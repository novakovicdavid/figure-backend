use std::marker::PhantomData;
use async_trait::async_trait;
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Algorithm::Argon2id;
use argon2::password_hash::SaltString;

use lazy_static::lazy_static;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use crate::server_errors::ServerError;
use rand_core::OsRng;
use uuid::Uuid;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::dtos::session_dtos::Session;
use crate::repositories::traits::{ProfileRepositoryTrait, SessionRepositoryTrait, TransactionCreatorTrait, TransactionTrait, UserRepositoryTrait};
use crate::services::traits::UserServiceTrait;

lazy_static! {
    static ref EMAIL_REGEX: Regex =
    Regex::new("^[a-zA-Z0-9_+&*-]+(?:\\.[a-zA-Z0-9_+&*-]+)*@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,}$").unwrap();
    static ref USERNAME_REGEX: Regex =
    Regex::new("^[a-zA-Z0-9]+-*[a-zA-Z0-9]+?$").unwrap();
}

#[derive(Clone)]
pub struct UserService<TC, T, U, P, S> {
    user_repository: U,
    profile_repository: P,
    session_repository: S,
    transaction_creator: TC,
    marker: PhantomData<T>,
}

impl<TC, T, U, P, S> UserService<TC, T, U, P, S>
    where
        TC: TransactionCreatorTrait<T>,
        T: TransactionTrait,
        U: UserRepositoryTrait<T>,
        P: ProfileRepositoryTrait<T>,
        S: SessionRepositoryTrait,
{
    pub fn new(transaction_creator: TC, user_repository: U, profile_repository: P, session_repository: S) -> Self {
        UserService {
            user_repository,
            profile_repository,
            session_repository,
            transaction_creator,
            marker: PhantomData::default(),
        }
    }
}

#[async_trait]
impl<TC, T, U, P, S> UserServiceTrait for UserService<TC, T, U, P, S>
    where TC: TransactionCreatorTrait<T>, T: TransactionTrait,
          U: UserRepositoryTrait<T>, P: ProfileRepositoryTrait<T>, S: SessionRepositoryTrait {
    async fn signup_user(&self, email: String, password: String, username: String) -> Result<(ProfileDTO, Session), ServerError> {
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

        match self.transaction_creator.create().await {
            Ok(mut transaction) => {
                let user = self.user_repository.create(Some(&mut transaction), email, password_hash).await?;
                let profile = self.profile_repository.create(Some(&mut transaction), username, user.id).await?;
                transaction.commit().await?;

                let session = Session::new(
                    Uuid::new_v4().to_string(),
                    user.id,
                    profile.id,
                    Some(86400),
                );

                let session = self.session_repository.create(session).await?;
                Ok((ProfileDTO::from(profile), session))
            }
            Err(e) => Err(e)
        }
    }

    async fn authenticate_user(&self, email: String, password: String) -> Result<(ProfileDTO, Session), ServerError> {
        let user = match self.user_repository.find_one_by_email(None, email).await {
            Ok(user) => user,
            Err(_e) => return Err(ServerError::UserWithEmailNotFound),
        };
        let parsed_hash = match PasswordHash::new(&user.password) {
            Ok(hash) => hash,
            Err(e) => {
                return Err(ServerError::InternalError(e.into()));
            }

        };
        if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_err() {
            return Err(ServerError::WrongPassword);
        }
        let profile = match self.profile_repository.find_by_user_id(None, user.id).await {
            Ok(profile) => profile,
            Err(e) => return Err(ServerError::InternalError(e.into())),
        };

        let session = match self.session_repository.create(Session::new(Uuid::new_v4().to_string(), user.id, profile.id, Some(86400))).await {
            Ok(session) => session,
            Err(e) => return Err(ServerError::InternalError(e.into())),
        };
        Ok((ProfileDTO::from(profile), session))
    }
}

// Valid email test (OWASP Regex + maximum length of 60 graphemes
fn is_email_valid(email: &str) -> bool {
    let email_count = email.graphemes(true).count();
    EMAIL_REGEX.is_match(email) && (3..=60).contains(&email_count)
}

// Valid username test
// (alphanumerical, optionally a dash surrounded by alphanumerical characters, 15 character limit)
fn is_username_valid(username: &str) -> bool {
    let username_count = username.graphemes(true).count();
    USERNAME_REGEX.is_match(username) && (3..=15).contains(&username_count)
}

pub fn hash_password(password: &str, with_checks: bool) -> Result<String, ServerError> {
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
            return Err(ServerError::InternalError(e.into()));
        }
    };
    let password_hash = match Argon2::new(Argon2id, argon2::Version::V0x13, argon2_params).hash_password(password.as_ref(), &password_salt) {
        Ok(password_hash) => password_hash,
        Err(e) => {
            return Err(ServerError::InternalError(e.into()));
        }
    };
    Ok(password_hash.to_string())
}