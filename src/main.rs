mod entities;
mod server_errors;
mod routes;
mod tests;
mod content_store;
mod services;
mod repositories;
mod context;
mod utilities;
mod environment;
mod domain;
mod infrastructure;
mod layers;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use axum::{Extension, middleware, Router};
use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use axum::routing::post;
use redis::aio::ConnectionManager;
use sqlx::{Pool, Postgres};
use tokio::task;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_cookies::CookieManagerLayer;
use tower_http::limit::RequestBodyLimitLayer;
use url::Url;
use tracing::info;
use http::Method;
use http::header::{ACCEPT, CONTENT_TYPE};
use crate::layers::auth_layer::authenticate;
use crate::content_store::S3Storage;
use crate::context::{Context, ContextTrait, RepositoryContext, ServiceContext};
use crate::entities::dtos::session_dtos::SessionOption;
use crate::environment::Environment;
use crate::repositories::figure_repository::FigureRepository;
use crate::repositories::profile_repository::ProfileRepository;
use crate::repositories::session_repository::SessionRepository;
use crate::repositories::transaction::PostgresTransactionManager;
use crate::repositories::user_repository::UserRepository;
use crate::routes::authentication_routes::{load_session, sign_in, sign_out, sign_up};
use crate::routes::figure_routes::{browse_figures, browse_figures_from_profile, browse_figures_from_profile_starting_from_figure_id, browse_figures_starting_from_figure_id, get_figure, get_total_figures_by_profile, get_total_figures_count, landing_page_figures, upload_figure};
use crate::routes::misc_routes::healthcheck;
use crate::routes::profile_routes::{get_profile, get_total_profiles_count, update_profile};
use crate::services::figure_service::FigureService;
use crate::services::profile_service::ProfileService;
use crate::services::user_service::UserService;
use crate::utilities::logging::init_logging;
use crate::utilities::secure_rand_generator::ChaCha20;
use crate::layers::correlation_id_layer::{correlation_id_extension};
use crate::layers::tracing_layer::create_tracing_layer;

pub struct ServerState<C: ContextTrait> {
    context: C,
    domain: String,
}

impl<C: ContextTrait> ServerState<C> {
    pub fn new(context: C, domain: String) -> Self {
        Self {
            context,
            domain,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let time_to_start = Instant::now();

    let env = Environment::new()?;

    init_logging(env.loki_host, env.loki_url).expect("Failed to initialize logging!");

    info!("Connecting to database...");
    let db_pool_future = task::spawn(async move {
        let time = Instant::now();
        Pool::<Postgres>::connect(&env.database_url).await
            .map(|pool| {
                info!("Connected to database in {}ms...", time.elapsed().as_millis());
                pool
            })
    });

    info!("Connecting to session store...");
    let client = redis::Client::open(env.redis_url)?;
    let session_store_connection_future = task::spawn(async move {
        let time = Instant::now();
        ConnectionManager::new(client).await
            .map(|session_store| {
                info!("Connected to session store in {}ms...", time.elapsed().as_millis());
                session_store
            })
    });

    let content_store = S3Storage::new_store(
        env.s3_app_id, env.s3_app_key, env.s3_region,
        env.s3_endpoint, env.s3_base_storage_url, env.s3_bucket,
    );

    info!("Setting up CORS...");
    let cors = create_app_cors([env.origin.parse()?]);
    info!("Allowed origin (CORS): {}", env.origin);

    // Struct containing optional user session from a request
    let authentication_extension = create_authentication_extension();

    let domain = Url::parse(&env.origin)?.host_str().unwrap().to_string();
    info!("Domain parsed from origin: {}", domain);

    info!("Waiting for stores...");
    let db_pool = db_pool_future.await??;
    let session_store = session_store_connection_future.await??;

    info!("Creating state...");
    let server_state = create_state(db_pool, session_store, content_store, domain);

    info!("Setting up routes and layers...");
    let app = create_app(server_state, cors, authentication_extension);

    let server_port = env.server_port;
    let addr = SocketAddr::from(([0, 0, 0, 0], server_port));

    info!("Starting Axum...");
    let axum_server = axum::Server::bind(&addr)
        .serve(app.into_make_service());

    info!("Server is up at port {}", server_port);
    info!("Ready to serve in {}ms", time_to_start.elapsed().as_millis());

    axum_server.await?;
    Ok(())
}

fn create_app<C: ContextTrait + 'static>(server_state: Arc<ServerState<C>>, cors: CorsLayer, authentication_extension: SessionOption) -> Router {
    Router::new()
        .route("/profile/update", post(update_profile))
        .route("/figures/upload", post(upload_figure))
        // Disable the default limit
        .layer(DefaultBodyLimit::disable())
        // Set a different limit
        .layer(RequestBodyLimitLayer::new(5 * 1000000))

        .route("/healthcheck", get(healthcheck))
        .route("/users/signup", post(sign_up))
        .route("/users/signin", post(sign_in))
        .route("/session/invalidate", post(sign_out))
        .route("/session/load", get(load_session))
        .route("/figures/:id", get(get_figure))
        .route("/figures/browse", get(browse_figures))
        .route("/figures/landing-page", get(landing_page_figures))
        .route("/figures/browse/:starting_from_figure_id", get(browse_figures_starting_from_figure_id))
        .route("/profile/:profile_id/browse", get(browse_figures_from_profile))
        .route("/profile/:profile_id/total_figures", get(get_total_figures_by_profile))
        .route("/profile/:profile_id/browse/:starting_from_figure_id", get(browse_figures_from_profile_starting_from_figure_id))
        .route("/profiles/:id", get(get_profile))
        .route("/profiles/count", get(get_total_profiles_count))
        .route("/figures/count", get(get_total_figures_count))

        .layer(middleware::from_fn_with_state(server_state.clone(), authenticate))
        .layer(Extension(authentication_extension))
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .with_state(server_state)
        .layer(create_tracing_layer())
        .layer(middleware::from_fn(correlation_id_extension))
}

fn create_state(db_pool: Pool<Postgres>, session_store: ConnectionManager, content_store: S3Storage, domain: String) -> Arc<ServerState<impl ContextTrait>> {
    // Initialize repositories
    let transaction_starter = PostgresTransactionManager::new(db_pool.clone());
    let user_repository = UserRepository::new(db_pool.clone());
    let profile_repository = ProfileRepository::new(db_pool.clone());
    let figure_repository = FigureRepository::new(db_pool.clone());
    let session_repository = SessionRepository::new(session_store);

    // Initialize utilities
    let secure_random_generator = ChaCha20::new();

    // Initialize services
    let user_service = UserService::new(
        transaction_starter.clone(), user_repository.clone(),
        profile_repository.clone(), session_repository.clone(),
        secure_random_generator);
    let profile_service = ProfileService::new(profile_repository.clone(), content_store.clone());
    let figure_service = FigureService::new(figure_repository.clone(), content_store);

    // Create service and repository contexts
    let repository_context = RepositoryContext::new(user_repository, profile_repository, figure_repository, session_repository, transaction_starter);
    let service_context = ServiceContext::new(user_service, profile_service, figure_service);

    // Combine contexts
    let context = Context::new(service_context, repository_context);

    // Resulting state
    Arc::new(ServerState::new(context, domain))
}

fn create_authentication_extension() -> SessionOption {
    SessionOption {
        session_opt: None
    }
}

fn create_app_cors<T: Into<AllowOrigin>>(origins: T) -> CorsLayer {
    CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE])
        .allow_origin(origins)
}