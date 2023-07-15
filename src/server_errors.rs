use serde::{Serialize};
use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tracing::error;

#[derive(Debug)]
pub enum ServerError {
    InvalidEmail,
    InvalidUsername,
    PasswordTooShort,
    PasswordTooLong,
    EmailAlreadyInUse,
    UsernameAlreadyTaken,
    UserWithEmailNotFound,
    WrongPassword,
    ResourceNotFound,
    // No session cookie received
    NoSessionReceived,
    InvalidImage,
    MissingFieldInForm,
    InvalidMultipart,
    ImageDimensionsTooLarge,
    InternalError(anyhow::Error),
}

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    pub(crate) error: &'a str,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ServerError::InvalidEmail => "invalid-email",
            ServerError::InvalidUsername => "invalid-username",
            ServerError::PasswordTooShort => "password-too-short",
            ServerError::PasswordTooLong => "password-too-long",
            ServerError::EmailAlreadyInUse => "email-already-in-use",
            ServerError::UsernameAlreadyTaken => "username-already-taken",
            ServerError::UserWithEmailNotFound => "user-with-email-not-found",
            ServerError::WrongPassword => "wrong-password",
            ServerError::ResourceNotFound => "resource-not-found",
            ServerError::NoSessionReceived => "no-session-received",
            ServerError::InvalidImage => "invalid-image",
            ServerError::MissingFieldInForm => "missing-field-in-form",
            ServerError::InvalidMultipart => "invalid-multipart",
            ServerError::ImageDimensionsTooLarge => "image-dimensions-too-large",
            ServerError::InternalError(error) => {
                error!("Internal server error: {}\n{}", error, error.backtrace());
                "internal-server-error"
            }
        };
        write!(f, "{}", message)
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            ServerError::InvalidEmail => StatusCode::BAD_REQUEST,
            ServerError::InvalidUsername => StatusCode::BAD_REQUEST,
            ServerError::PasswordTooShort => StatusCode::BAD_REQUEST,
            ServerError::PasswordTooLong => StatusCode::BAD_REQUEST,
            ServerError::EmailAlreadyInUse => StatusCode::BAD_REQUEST,
            ServerError::UsernameAlreadyTaken => StatusCode::BAD_REQUEST,
            ServerError::UserWithEmailNotFound => StatusCode::NOT_FOUND,
            ServerError::WrongPassword => StatusCode::BAD_REQUEST,
            ServerError::ResourceNotFound => StatusCode::NOT_FOUND,
            ServerError::NoSessionReceived => StatusCode::BAD_REQUEST,
            ServerError::InvalidImage => StatusCode::BAD_REQUEST,
            ServerError::MissingFieldInForm => StatusCode::BAD_REQUEST,
            ServerError::InvalidMultipart => StatusCode::BAD_REQUEST,
            ServerError::ImageDimensionsTooLarge => StatusCode::BAD_REQUEST,
            ServerError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (
            status_code,
            Json(ErrorResponse {
                error: &self.to_string()
            })
        ).into_response()
    }
}

impl std::error::Error for ServerError {}

impl PartialEq for ServerError {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}