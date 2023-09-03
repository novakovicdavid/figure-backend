use std::marker::PhantomData;
use std::sync::Arc;
use async_trait::async_trait;
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Algorithm::Argon2id;
use argon2::password_hash::SaltString;
use unicode_segmentation::UnicodeSegmentation;
use crate::server_errors::ServerError;
use rand_core::OsRng;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::dtos::session_dtos::Session;
use crate::repositories::traits::{ProfileRepositoryTrait, SessionRepositoryTrait, TransactionManagerTrait, TransactionTrait, UserRepositoryTrait};
use crate::services::traits::UserServiceTrait;
use crate::utilities::traits::RandomNumberGenerator;
use interpol::format as iformat;
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
    async fn signup_user(&self, email: String, password: String, username: String) -> Result<(ProfileDTO, Session), ServerError> {
        User::validate_email(&email)?;
        User::validate_password(&password)?;
        Profile::validate_username(&username)?;

        if self.user_repository.find_one_by_email(None, email.clone()).await.is_ok() {
            return Err(ServerError::EmailAlreadyInUse);
        }
        let password_hash_result = hash_password(&password);
        let password_hash = match password_hash_result {
            Ok(hash) => hash,
            Err(e) => return Err(e)
        };

        let mut transaction = self.transaction_creator.create().await?;

        let user = User::new(0, email, password_hash, "user".to_string())?;
        let user = self.user_repository.create(Some(&mut transaction), user).await?;

        let profile = Profile::new(0, username, None, None, None, None, user.get_id())?;
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

    async fn authenticate_user(&self, email: String, password: String) -> Result<(ProfileDTO, Session), ServerError> {
        User::validate_email(&email)?;
        User::validate_password(&password)?;
        let user = self.user_repository.find_one_by_email(None, email).await?;

        let parsed_hash = match PasswordHash::new(user.get_password()) {
            Ok(hash) => hash,
            Err(e) => {
                return Err(ServerError::InternalError(Arc::new(e.into())));
            }
        };
        if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_err() {
            return Err(ServerError::WrongPassword);
        }
        let profile = match self.profile_repository.find_by_user_id(None, user.get_id()).await {
            Ok(profile) => profile,
            Err(e) => return Err(ServerError::InternalError(Arc::new(anyhow::Error::from(e)
                .context(iformat!("Profile associated with user (id: {user.get_id()}) not found."))))),
        };

        let session = self.session_repository.create(Session::new(self.secure_random_generator.generate()?.to_string(), user.get_id(), profile.get_id(), Some(86400))).await?;
        Ok((ProfileDTO::from(profile), session))
    }
}

pub fn hash_password(password: &str) -> Result<String, ServerError> {
    let password_salt = SaltString::generate(&mut OsRng);
    let argon2_params = match Params::new(8192, 5, 1, Some(32)) {
        Ok(argon2_params) => argon2_params,
        Err(e) => {
            return Err(ServerError::InternalError(Arc::new(e.into())));
        }
    };
    let password_hash = match Argon2::new(Argon2id, argon2::Version::V0x13, argon2_params).hash_password(password.as_ref(), &password_salt) {
        Ok(password_hash) => password_hash,
        Err(e) => {
            return Err(ServerError::InternalError(Arc::new(e.into())));
        }
    };
    Ok(password_hash.to_string())
}