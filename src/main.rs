mod database;
mod session_store;
mod auth_layer;
mod entities;
mod server_errors;
mod routes;
mod tests;

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use axum::{Extension, middleware, Router};
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::Method;
use axum::routing::get;
use axum::routing::post;
use futures::FutureExt;
use log::info;
use crate::database::{Database, get_database_connection};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_cookies::CookieManagerLayer;
use url::Url;
use crate::auth_layer::authenticate;
use crate::entities::types::Id;
use crate::routes::authentication_routes::{load_session, signin_user, signout_user, signup_user};
use crate::routes::figure_routes::get_figure;
use crate::routes::misc_routes::healthcheck;
use crate::session_store::{SessionStore, SessionStoreConnection};

pub struct ServerState {
    database: Database,
    session_store: SessionStore,
    domain: String,
}

#[derive(Clone, Debug)]
pub struct Session {
    id: String,
    user_id: Id,
    profile_id: Id,
}

#[derive(Clone, Debug)]
pub struct SessionOption {
    session: Option<Session>,
}

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let time_to_start = Instant::now();
    env_logger::init();

    info!("Connecting to database...");
    let database_url = env::var("DATABASE_URL")?;
    let database = get_database_connection(&database_url)
        .then(|database| async {
            info!("Connected to database...");
            database
        });

    info!("Connecting to session store...");
    let session_store_url = env::var("REDIS_URL")?;
    let session_store = SessionStoreConnection::new(&session_store_url)
        .then(|session_store| async {
            info!("Connected to session store...");
            session_store
        });

    info!("Setting up CORS...");
    let cors = create_app_cors([env::var("ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string()).parse()?]);

    // Struct containing optional user session from a request
    let authentication_extension = create_authentication_extension();

    let domain = Url::parse(&env::var("ORIGIN")
        .unwrap_or_else(|_| "http://localhost".to_string()))?.host_str().unwrap().to_string();

    info!("Waiting for stores...");
    let database = database.await;
    let session_store = session_store.await;
    let server_state = create_server_state(database, session_store, domain);

    info!("Setting up routes and layers...");
    let app = create_app(server_state, cors, authentication_extension);

    info!("Starting Axum...");
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string()).parse::<i32>()?;
    let addr = SocketAddr::from(([127, 0, 0, 1], env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string()).parse()?));
    let axum_server = axum::Server::bind(&addr)
        .serve(app.into_make_service());
    info!("Server is up at port {}", server_port);
    info!("Ready to serve in {}ms", time_to_start.elapsed().as_millis());
    axum_server.await?;
    Ok(())
}

fn create_app(server_state: Arc<ServerState>, cors: CorsLayer, authentication_extension: SessionOption) -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/figures/:id", get(get_figure))
        .route("/users/signup", post(signup_user))
        .route("/users/signin", post(signin_user))
        .route("/session/invalidate", post(signout_user))
        .route("/session/load", get(load_session))

        .layer(middleware::from_fn_with_state(server_state.clone(), authenticate))
        .layer(Extension(authentication_extension))
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .with_state(server_state)
}

fn create_server_state(database: Database, session_store: SessionStore, domain: String) -> Arc<ServerState> {
    Arc::new(ServerState {
        database,
        session_store,
        domain,
    })
}

fn create_authentication_extension() -> SessionOption {
    SessionOption {
        session: None
    }
}

fn create_app_cors<T: Into<AllowOrigin>>(origins: T) -> CorsLayer {
    CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE])
        .allow_origin(origins)
}