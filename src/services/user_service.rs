use std::marker::PhantomData;
use async_trait::async_trait;
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Algorithm::Argon2id;
use argon2::password_hash::SaltString;

use lazy_static::lazy_static;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use crate::entities::user::User;
use crate::server_errors::ServerError;
use rand_core::OsRng;
use crate::entities::profile::Profile;
use crate::repositories::profile_repository::ProfileRepositoryTrait;
use crate::repositories::transaction::TransactionTrait;
use crate::repositories::user_repository::UserRepositoryTrait;


lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new("^[a-zA-Z0-9_+&*-]+(?:\\.[a-zA-Z0-9_+&*-]+)*@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,}$").unwrap();
    static ref USERNAME_REGEX: Regex = Regex::new("^[a-zA-Z0-9]+-*[a-zA-Z0-9]+?$").unwrap();
}

#[async_trait]
pub trait UserServiceTrait: Send + Sync + Clone {
    async fn signup_user(&self, email: String, password: String, username: String) -> Result<(User, Profile), ServerError<String>>;
    async fn authenticate_user(&self, email: String, password: String) -> Result<(User, Profile), ServerError<String>>;
}

#[derive(Clone)]
pub struct UserService<U: UserRepositoryTrait, P: ProfileRepositoryTrait> {
    user_repository: U,
    profile_repository: P,
}

impl<U: UserRepositoryTrait, P: ProfileRepositoryTrait> UserService<U, P> {
    pub fn new(user_repository: U, profile_repository: P) -> Self {
        UserService {
            user_repository,
            profile_repository,
        }
    }
}

#[async_trait]
impl<U: UserRepositoryTrait, P: ProfileRepositoryTrait> UserServiceTrait for UserService<U, P> {
    async fn signup_user(&self, email: String, password: String, username: String) -> Result<(User, Profile), ServerError<String>> {
        if !is_email_valid(&email) {
            return Err(ServerError::InvalidEmail);
        }
        if !is_username_valid(&username) {
            return Err(ServerError::InvalidUsername);
        }

        let password_hash_result = hash_password(&password, true);
        let password_hash = match password_hash_result {
            Ok(hash) => hash,
            Err(e) => return Err(e)
        };

        match self.user_repository.start_transaction().await {
            Ok(mut transaction) => {
                let user_result = self.user_repository.create(Some(&mut transaction), email.to_string(), password_hash.to_string()).await;
                if let Ok(user) = user_result {
                    let profile_result = self.profile_repository.create(Some(&mut transaction), username, user.id).await;
                    if let Ok(profile) = profile_result {
                        return match transaction.commit().await {
                            Ok(_) => Ok((user, profile)),
                            Err(_) => Err(ServerError::TransactionFailed)
                        };
                    }
                    return Err(ServerError::UsernameAlreadyTaken);
                }
                Err(ServerError::EmailAlreadyInUse)
            }
            Err(e) => Err(e)
        }
    }

    async fn authenticate_user(&self, email: String, password: String) -> Result<(User, Profile), ServerError<String>> {
        if let Ok(user) = self.user_repository.get_user_by_email(None, email).await {
            let parsed_hash = match PasswordHash::new(&user.password) {
                Ok(hash) => hash,
                Err(e) => {
                    return Err(ServerError::InternalError(e.to_string()));
                }
            };
            let password_verification = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
            if password_verification.is_ok() {
                match self.profile_repository.find_by_user_id(None, user.id).await {
                    Ok(profile_result) => Ok((user, profile_result)),
                    Err(e) => return Err(ServerError::InternalError(e.to_string()))
                }
            } else {
                Err(ServerError::WrongPassword)
            }
        } else {
            Err(ServerError::UserWithEmailNotFound)
        }
    }
}

// Valid email test (OWASP Regex + maximum length of 60 graphemes
fn is_email_valid(email: &str) -> bool {
    EMAIL_REGEX.is_match(email) || email.graphemes(true).count() > 60
}

// Valid username test
// (alphanumerical, optionally a dash surrounded by alphanumerical characters, 15 character limit)
fn is_username_valid(username: &str) -> bool {
    USERNAME_REGEX.is_match(username) || username.graphemes(true).count() > 15
}

fn hash_password(password: &str, with_checks: bool) -> Result<String, ServerError<String>> {
    if with_checks {
        let password_length = password.graphemes(true).count();
        if password_length < 8 {
            return Err(ServerError::PasswordTooShort);
        }
        if password_length > 60 {
            return Err(ServerError::PasswordTooLong);
        }
    }

    let password_salt = SaltString::generate(&mut OsRng);
    let argon2_params = match Params::new(8192, 5, 1, Some(32)) {
        Ok(argon2_params) => argon2_params,
        Err(e) => {
            return Err(ServerError::InternalError(e.to_string()));
        }
    };
    let password_hash = match Argon2::new(Argon2id, argon2::Version::V0x13, argon2_params).hash_password(password.as_ref(), &password_salt) {
        Ok(password_hash) => password_hash,
        Err(e) => {
            return Err(ServerError::InternalError(e.to_string()));
        }
    };
    Ok(password_hash.to_string())
}