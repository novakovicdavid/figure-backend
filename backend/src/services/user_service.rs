use std::marker::PhantomData;
use async_trait::async_trait;
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Algorithm::Argon2id;
use argon2::password_hash::{Error, SaltString};
use crate::server_errors::ServerError;
use rand_core::OsRng;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::dtos::session_dtos::Session;
use crate::repositories::traits::{ProfileRepositoryTrait, SessionRepositoryTrait, TransactionManagerTrait, TransactionTrait, UserRepositoryTrait};
use crate::services::traits::UserServiceTrait;
use crate::utilities::traits::RandomNumberGenerator;
use crate::domain::models::profile::Profile;
use crate::domain::models::user::User;


#[derive(Clone)]
pub struct UserService<TC, T, U, P, S, R> {
    transaction_creator: TC,
    marker: PhantomData<T>,
    user_repository: U,
    profile_repository: P,
    session_repository: S,
    secure_random_generator: R,
}

impl<TC, T, U, P, S, R> UserService<TC, T, U, P, S, R>
    where
        TC: TransactionManagerTrait<T>,
        T: TransactionTrait,
        U: UserRepositoryTrait<T>,
        P: ProfileRepositoryTrait<T>,
        S: SessionRepositoryTrait,
        R: RandomNumberGenerator,
{
    pub fn new(transaction_creator: TC, user_repository: U, profile_repository: P, session_repository: S, secure_random_generator: R) -> Self {
        UserService {
            user_repository,
            profile_repository,
            session_repository,
            transaction_creator,
            secure_random_generator,
            marker: PhantomData::default(),
        }
    }
}

#[async_trait]
impl<TC, T, U, P, S, R> UserServiceTrait for UserService<TC, T, U, P, S, R>
    where TC: TransactionManagerTrait<T>, T: TransactionTrait,
          U: UserRepositoryTrait<T>, P: ProfileRepositoryTrait<T>, S: SessionRepositoryTrait,
          R: RandomNumberGenerator {
    async fn sign_up(&self, email: &str, password: &str, username: &str) -> Result<(ProfileDTO, Session), ServerError> {
        User::validate_email(email)?;
        User::validate_password(password)?;
        Profile::validate_username(username)?;

        if self.user_repository.find_one_by_email(None, email).await.is_ok() {
            return Err(ServerError::EmailAlreadyInUse);
        }

        let password_hash = hash_password(password)?;

        let mut transaction = self.transaction_creator.create().await?;

        let user = User::new(0, email.to_string(), password_hash, "user".to_string())?;
        let user = self.user_repository.create(Some(&mut transaction), user).await?;

        let profile = Profile::new(0, username.to_string(), None, None, None, None, user.get_id())?;
        let profile = self.profile_repository.create(Some(&mut transaction), profile).await?;

        transaction.commit().await?;

        let session = Session::new(
            self.secure_random_generator.generate()?.to_string(),
            user.get_id(),
            profile.get_id(),
            Some(86400),
        );

        let session = self.session_repository.create(session).await?;
        Ok((ProfileDTO::from(profile), session))
    }

    async fn sign_in(&self, email: &str, password: &str) -> Result<(ProfileDTO, Session), ServerError> {
        User::validate_email(email)?;
        User::validate_password(password)?;

        let user = self.user_repository.find_one_by_email(None, email).await?;

        let parsed_hash = PasswordHash::new(user.get_password())
            .map_err(|e| ServerError::InternalError(e.into()))?;

        verify_password(password, parsed_hash)?;

        let profile = self.profile_repository.find_by_user_id(None, user.get_id()).await?;

        let session = self.session_repository.create(Session::new(self.secure_random_generator.generate()?.to_string(), user.get_id(), profile.get_id(), Some(86400))).await?;

        Ok((ProfileDTO::from(profile), session))
    }
}

pub fn hash_password(password: &str) -> Result<String, ServerError> {
    let password_salt = SaltString::generate(&mut OsRng);
    let argon2_params = Params::new(8192, 5, 1, Some(32))
        .map_err(|e| ServerError::InternalError(e.into()))?;

    let password_hash = Argon2::new(Argon2id, argon2::Version::V0x13, argon2_params).hash_password(password.as_ref(), &password_salt)
        .map_err(|e| ServerError::InternalError(e.into()))?;
    Ok(password_hash.to_string())
}

fn verify_password(password: &str, saved_hash: PasswordHash) -> Result<(), ServerError> {
    Argon2::default().verify_password(password.as_bytes(), &saved_hash)
        .map_err(|e| match e {
            Error::Password => ServerError::WrongPassword,
            _ => ServerError::InternalError(e.into())
        })
}