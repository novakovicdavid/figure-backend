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
    let key_id = env::var("S3_APP_ID").expect("No S3_APP_ID env found");
    let app_key = env::var("S3_APP_KEY").expect("No S3_APP_KEY env found");
    let s3_region = env::var("S3_REGION").expect("No S3_REGION env found");
    let bucket_endpoint = env::var("S3_ENDPOINT").expect("No S3_ENDPOINT env found");
    let base_storage_url = env::var("S3_BASE_STORAGE_URL").expect("No S3_BASE_STORAGE_URL env found");
    let bucket = env::var("S3_BUCKET").expect("No S3_BUCKET env found");
    let content_store = S3Storage::new_store(key_id, app_key, s3_region, bucket_endpoint, base_storage_url, bucket);
    let domain = "localhost";

    Ok(create_app(
        create_server_state(database, session_store, content_store, domain.to_string()),
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
            "title": "My other cat",
            "width": 4128,
            "height": 3096,
            "profile": {
                "id": 1,
                "username": "one",
                "display_name": "hi",
            },
            "url": "https://cdn.figure.novakovic.be/35357ff7-f1c0-4264-9c2a-98119ac6eaed",
            "description": "o.o"
        }
    }));
    Ok(())
}

#[tokio::test]
async fn test_get_figure_non_existing() -> Result<(), Error> {
    let app = setup().await?;
    let response = app
        .oneshot(Request::builder().method("GET").uri("/figures/43545345345345").body(Body::empty())?)
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
                "email": "five@five.five",
                "username": "five",
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
            "id": 5,
            "username": "five",
            "display_name": None as Option<String>
        }
    }));



    Ok(())
}