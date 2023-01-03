use std::error::Error;
use std::fmt::{Display, Formatter};

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
    InternalError
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
            ServerError::InternalError => "internal-error"
        };
        write!(f, "{}", message)
    }
}

impl Error for ServerError {}