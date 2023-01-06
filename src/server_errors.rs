use std::error::Error;
use std::fmt::{Display, Formatter};

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
    InternalError(T)
}

impl Display for ServerError<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ServerError::InvalidEmail => "invalid-email".to_string(),
            ServerError::InvalidUsername => "invalid-username".to_string(),
            ServerError::PasswordTooShort => "password-too-short".to_string(),
            ServerError::PasswordTooLong => "password-too-long".to_string(),
            ServerError::EmailAlreadyInUse => "email-already-in-use".to_string(),
            ServerError::UsernameAlreadyTaken => "username-already-taken".to_string(),
            ServerError::UserWithEmailNotFound => "user-with-email-not-found".to_string(),
            ServerError::WrongPassword => "wrong-password".to_string(),
            ServerError::ResourceNotFound => "resource-not-found".to_string(),
            ServerError::InternalError(error) => format!("internal-error: {}", error)
        };
        write!(f, "{}", message)
    }
}

impl Error for ServerError<String> {}