use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn healthcheck() -> Response {
    StatusCode::OK.into_response()
}