use lazy_static::lazy_static;
use regex::Regex;
use serde::{Serialize};
use unicode_segmentation::UnicodeSegmentation;
use crate::domain::models::profile::Profile;
use crate::domain::models::types::IdType;
use crate::server_errors::ServerError;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct User {
    id: IdType,
    email: String,
    password: String,
    role: String,
}

impl User {
    pub fn new(id: IdType, email: String, password: String, role: String) -> Result<Self, ServerError> {
        Self::validate_email(&email)?;

        Ok(Self::new_raw(id, email, password, role))
    }

    pub fn new_raw(id: IdType, email: String, password: String, role: String) -> Self {
        Self {
            id,
            email: email.to_lowercase(),
            password,
            role,
        }
    }

    // Valid email test (OWASP Regex + maximum length of 60 graphemes
    pub fn validate_email(email: &str) -> Result<(), ServerError> {
        let graphemes = email.graphemes(true);
        let mut count = 0;
        for _ in graphemes {
            count += 1;
            if count > 60 {
                return Err(ServerError::InvalidEmail)
            }
        }
        if count < 3 {
            return Err(ServerError::InvalidEmail)
        }

        if !EMAIL_REGEX.is_match(email) {
            return Err(ServerError::InvalidEmail)
        }

        Ok(())
    }

    pub fn validate_password(password: &str) -> Result<(), ServerError> {
        let password_length = password.graphemes(true).count();

        if password_length < 8 {
            return Err(ServerError::PasswordTooShort);
        }

        if password_length > 128 {
            return Err(ServerError::PasswordTooLong);
        }

        Ok(())
    }

    pub fn get_id(&self) -> IdType {
        self.id
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    pub fn set_id(&mut self, id: IdType) {
        self.id = id;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn set_role(&mut self, role: String) {
        self.role = role;
    }
}

lazy_static! {
    static ref EMAIL_REGEX: Regex =
    Regex::new("^[a-zA-Z0-9_+&*-]+(?:\\.[a-zA-Z0-9_+&*-]+)*@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,}$").unwrap();
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserAndProfileFromQuery {
    #[sqlx(flatten)]
    pub user: User,
    #[sqlx(flatten)]
    pub profile: Profile,
}