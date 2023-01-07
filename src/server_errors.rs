use serde::{Serialize};
use std::fmt::{Display, Formatter};
use axum::body::BoxBody;
use axum::http::{Response, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use log::error;
use sqlx::Error;

#[derive(Debug)]
pub enum ServerError<T: ToString> {
    InvalidEmail,
    InvalidUsername,
    PasswordTooShort,
    PasswordTooLong,
    EmailAlreadyInUse,
    UsernameAlreadyTaken,
    UserWithEmailNotFound,
    WrongPassword,
    ResourceNotFound,
    ConstraintError,
    NoSessionFound,
    InternalError(T)
}

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    pub(crate) error: &'a str,
}

impl Display for ServerError<String> {
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
            ServerError::ConstraintError => "constraint-error",
            ServerError::NoSessionFound => "no-session-found",
            ServerError::InternalError(error) => {
                error!("Internal server error: {}", error);
                "internal-error"
            }
        };
        write!(f, "{}", message)
    }
}

impl ServerError<String> {
    pub fn parse_db_error(error: &Error) -> ServerError<String>{
        if let Error::Database(ref e) = error {
            // Constraint violation, email address likely already used
            if e.constraint().is_some() {
                return ServerError::ConstraintError;
            }
        }
        ServerError::InternalError(error.to_string())
    }

    pub fn into_response(self) -> Response<BoxBody> {
        let status_code = match self {
            ServerError::InvalidEmail => StatusCode::BAD_REQUEST,
            ServerError::InvalidUsername => StatusCode::BAD_REQUEST,
            ServerError::PasswordTooShort => StatusCode::BAD_REQUEST,
            ServerError::PasswordTooLong => StatusCode::BAD_REQUEST,
            ServerError::EmailAlreadyInUse => StatusCode::BAD_REQUEST,
            ServerError::UsernameAlreadyTaken => StatusCode::BAD_REQUEST,
            ServerError::UserWithEmailNotFound => StatusCode::BAD_REQUEST,
            ServerError::WrongPassword => StatusCode::BAD_REQUEST,
            ServerError::ResourceNotFound => StatusCode::BAD_REQUEST,
            ServerError::ConstraintError => StatusCode::BAD_REQUEST,
            ServerError::NoSessionFound => StatusCode::BAD_REQUEST,
            ServerError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR
        };
        (
            status_code,
            Json(ErrorResponse {
                error: &self.to_string()
            })
        ).into_response()
    }
}