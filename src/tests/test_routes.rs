use anyhow::Error;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::util::ServiceExt;
use crate::*;

async fn setup() -> Result<Router, Error> {
    let database = get_database_connection("postgres://postgres:mysecretpassword@localhost/postgres").await;
    let session_store = SessionStoreConnection::new("redis://localhost:6379").await;
    let domain = "localhost";

    Ok(create_app(
        create_server_state(database, session_store, domain.to_string()),
        create_app_cors(["http://localhost:3000".parse()?]),
        create_authentication_extension(),
    ))
}

#[tokio::test]
async fn healthcheck_test() -> Result<(), Error> {
    let app = setup().await?;

    let response = app
        .oneshot(Request::builder().uri("/healthcheck").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}