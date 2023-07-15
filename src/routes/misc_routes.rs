use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn healthcheck() -> StatusCode {
    StatusCode::OK
}