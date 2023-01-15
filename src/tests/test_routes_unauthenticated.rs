use anyhow::Error;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use hyper::header::SET_COOKIE;
use serde_json::{json, Value};
use tower::util::ServiceExt;
use crate::*;
use crate::server_errors::ServerError;

pub async fn setup() -> Result<Router, Error> {
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
async fn test_healthcheck() -> Result<(), Error> {
    let app = setup().await?;

    let response = app
        .oneshot(Request::builder().uri("/healthcheck").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}

#[tokio::test]
async fn test_get_figure() -> Result<(), Error> {
    let app = setup().await?;

    let response = app
        .oneshot(Request::builder().method("GET").uri("/figures/1").body(Body::empty())?)
        .await?;
    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = serde_json::from_slice(&body)?;

    assert_eq!(body, json!({
        "figure": {
            "id": 1,
            "title": "first",
            "width": 1268,
            "height": 951,
            "profile": {
                "id": 1,
                "username": "one",
                "display_name": None as Option<String>,
            },
            "url": "https://i.imgur.com/XpNCV7a.jpg",
            "description": None as Option<String>
        }
    }));
    Ok(())
}

#[tokio::test]
async fn test_get_figure_non_existing() -> Result<(), Error> {
    let app = setup().await?;
    let response = app
        .oneshot(Request::builder().method("GET").uri("/figures/2").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = serde_json::from_slice(&body)?;
    assert_eq!(body, json!({
        "error": ServerError::ResourceNotFound.to_string()
    }));
    Ok(())
}

#[tokio::test]
async fn test_signup_user() -> Result<(), Error> {
    let app = setup().await?;
    let response = app
        .oneshot(Request::builder()
            .method("POST")
            .uri("/users/signup")
            .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(ACCEPT, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                "email": "four@four.four",
                "username": "four",
                "password": "password"
            }).to_string()
            ))?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    // Very naive haha
    assert_eq!(response.headers().get(SET_COOKIE).unwrap().len(), "session_id=0aa614f2-b30e-4937-9f7c-6d58d9912746; HttpOnly; SameSite=Strict; Secure; Path=/; Domain=localhost".len());

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = serde_json::from_slice(&body)?;
    assert_eq!(body, json!({
        "profile": {
            "id": 4,
            "username": "four",
            "display_name": None as Option<String>
        }
    }));



    Ok(())
}