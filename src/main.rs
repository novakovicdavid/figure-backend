mod routes;
mod database;
mod session_store;
mod auth_layer;
mod entities;
mod server_errors;

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
use crate::routes::{get_figure, signin_user, signup_user};
use tower_http::cors::CorsLayer;
use tower_cookies::CookieManagerLayer;
use crate::auth_layer::authenticate;
use crate::entities::types::Id;
use crate::session_store::{SessionStore, SessionStoreConnection};

pub struct ServerState {
    database: Database,
    session_store: SessionStore
}

#[derive(Clone, Debug)]
pub struct Session {
    id: String,
    user_id: Id,
    profile_id: Id,
}

#[derive(Clone, Debug)]
pub struct SessionOption {
    session: Option<Session>
}

#[tokio::main]
async fn main() {
    let time_to_start = Instant::now();
    env_logger::init();


    info!("Connecting to database...");
    let database = get_database_connection(env::var("DATABASE_URL").unwrap())
        .then(|database| async {
            info!("Connected to database...");
            database
        });
    info!("Connecting to session store...");
    let session_store = SessionStoreConnection::new(env::var("REDIS_URL").unwrap())
        .then(|session_store| async {
            info!("Connected to session store...");
            session_store
        });

    info!("Setting up CORS...");
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE])
        .allow_origin(["http://localhost:3000".parse().unwrap()]);

    // Struct containing optional user session from a request
    let user_id_extension = SessionOption {
        session: None
    };

    info!("Waiting for stores...");
    let database = database.await;
    let session_store = session_store.await;
    let server_state = Arc::new(ServerState {
        database,
        session_store
    });

    info!("Setting up routes and layers...");
    let app = Router::new()
        .route("/figures/:id", get(get_figure))
        .route("/users/signup", post(signup_user))
        .route("/users/signin", post(signin_user))

        .layer(middleware::from_fn_with_state(server_state.clone(), authenticate))
        .layer(Extension(user_id_extension))
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .with_state(server_state);

    info!("Starting Axum...");
    let server_port = env::var("SERVER_PORT").unwrap_or("8000".to_string()).parse::<i32>().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], env::var("SERVER_PORT").unwrap_or("8000".to_string()).parse().unwrap()));
    let axum_server = axum::Server::bind(&addr)
        .serve(app.into_make_service());
    info!("Server is up at port {}", server_port);
    info!("Ready to serve in {}ms", time_to_start.elapsed().as_millis());
    axum_server.await.unwrap();
}