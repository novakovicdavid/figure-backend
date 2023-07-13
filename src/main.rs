mod auth_layer;
mod entities;
mod server_errors;
mod routes;
mod tests;
mod content_store;
mod services;
mod repositories;
mod context;

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use axum::{Extension, middleware, Router};
use axum::extract::DefaultBodyLimit;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::Method;
use axum::routing::get;
use axum::routing::post;
use futures::FutureExt;
use log::info;
use redis::aio::ConnectionManager;
use sqlx::{Pool, Postgres};
use tokio::task;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_cookies::CookieManagerLayer;
use tower_http::limit::RequestBodyLimitLayer;
use url::Url;
use crate::auth_layer::authenticate;
use crate::content_store::S3Storage;
use crate::context::{Context, RepositoryContext, ServiceContext};
use crate::entities::dtos::session_dtos::SessionOption;
use crate::repositories::figure_repository::FigureRepository;
use crate::repositories::profile_repository::ProfileRepository;
use crate::repositories::session_repository::SessionRepository;
use crate::repositories::transaction::{PostgresTransaction, PostgresTransactionCreator};
use crate::repositories::user_repository::UserRepository;
use crate::routes::authentication_routes::{load_session, signin_user, signout_user, signup_user};
use crate::routes::figure_routes::{browse_figures, browse_figures_from_profile, browse_figures_from_profile_starting_from_figure_id, browse_figures_starting_from_figure_id, get_figure, get_total_figures_by_profile, get_total_figures_count, landing_page_figures, upload_figure};
use crate::routes::misc_routes::healthcheck;
use crate::routes::profile_routes::{get_profile, get_total_profiles_count, update_profile};
use crate::services::figure_service::FigureService;
use crate::services::profile_service::ProfileService;
use crate::services::user_service::UserService;

pub struct ServerState {
    context: Context<PostgresTransaction>,
    domain: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let time_to_start = Instant::now();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL env found");
    info!("Connecting to database...");
    let db_pool_future = task::spawn(async move {
        Pool::<Postgres>::connect(&database_url).await
            .map(|pool| {
                info!("Connected to database...");
                pool
            })
    });


    let session_store_url = env::var("REDIS_URL").expect("No REDIS_URL env found");
    info!("Connecting to session store...");
    let client = redis::Client::open(session_store_url)?;
    let session_store_connection_future = task::spawn(ConnectionManager::new(client)
        .then(|session_store| async {
            info!("Connected to session store...");
            session_store
        }));

    let key_id = env::var("S3_APP_ID").expect("No S3_APP_ID env found");
    let app_key = env::var("S3_APP_KEY").expect("No S3_APP_KEY env found");
    let s3_region = env::var("S3_REGION").expect("No S3_REGION env found");
    let bucket_endpoint = env::var("S3_ENDPOINT").expect("No S3_ENDPOINT env found");
    let base_storage_url = env::var("S3_BASE_STORAGE_URL").expect("No S3_BASE_STORAGE_URL env found");
    let bucket = env::var("S3_BUCKET").expect("No S3_BUCKET env found");
    let content_store = S3Storage::new_store(key_id, app_key, s3_region, bucket_endpoint, base_storage_url, bucket);

    info!("Setting up CORS...");
    let origin = env::var("ORIGIN").expect("No ORIGIN env found");
    let cors = create_app_cors([origin.parse()?]);
    info!("Allowed origin (CORS): {}", origin);

    // Struct containing optional user session from a request
    let authentication_extension = create_authentication_extension();

    let domain = Url::parse(&env::var("ORIGIN")
        .unwrap_or_else(|_| "http://localhost".to_string()))?.host_str().unwrap().to_string();
    info!("Domain parsed from origin: {}", domain);

    info!("Waiting for stores...");
    let db_pool = db_pool_future.await??;
    let session_store = session_store_connection_future.await??;
    let server_state = create_state(db_pool, session_store, content_store, domain);

    info!("Setting up routes and layers...");
    let app = create_app(server_state, cors, authentication_extension);

    info!("Starting Axum...");
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string()).parse::<u16>()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], server_port));
    let axum_server = axum::Server::bind(&addr)
        .serve(app.into_make_service());
    info!("Server is up at port {}", server_port);
    info!("Ready to serve in {}ms", time_to_start.elapsed().as_millis());
    axum_server.await?;
    Ok(())
}

fn create_app(server_state: Arc<ServerState>, cors: CorsLayer, authentication_extension: SessionOption) -> Router {
    Router::new()
        .route("/profile/update", post(update_profile))
        .route("/figures/upload", post(upload_figure))
        // Disable the default limit
        .layer(DefaultBodyLimit::disable())
        // Set a different limit
        .layer(RequestBodyLimitLayer::new(5 * 1000000))

        .route("/healthcheck", get(healthcheck))
        .route("/users/signup", post(signup_user))
        .route("/users/signin", post(signin_user))
        .route("/session/invalidate", post(signout_user))
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
}

fn create_state(db_pool: Pool<Postgres>, session_store: ConnectionManager, content_store: S3Storage, domain: String) -> Arc<ServerState> {
    // Repositories
    let user_repository = UserRepository::new(db_pool.clone());
    let profile_repository = ProfileRepository::new(db_pool.clone());
    let figure_repository = FigureRepository::new(db_pool.clone());
    let session_repository = SessionRepository::new(session_store);

    // Transaction creator
    let transaction_starter = PostgresTransactionCreator::new(db_pool.clone());

    // Services
    let user_service = UserService::new(Box::new(transaction_starter.clone()), Box::new(user_repository.clone()), Box::new(profile_repository.clone()), Box::new(session_repository.clone()));
    let profile_service = ProfileService::new(Box::new(profile_repository.clone()), Box::new(content_store.clone()));
    let figure_service = FigureService::new(Box::new(figure_repository.clone()), Box::new(content_store));

    // Contexts that combine repositories and services
    let repository_context = RepositoryContext::new(Box::new(user_repository), Box::new(profile_repository), Box::new(figure_repository), Box::new(session_repository), Box::new(transaction_starter));
    let service_context = ServiceContext::new(Box::new(user_service), Box::new(profile_service), Box::new(figure_service));

    let context = Context::new(service_context, repository_context);

    Arc::new(ServerState {
        context,
        domain,
    })

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