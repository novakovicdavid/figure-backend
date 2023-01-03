use std::error::Error;
use std::fmt::{Debug};
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Algorithm::Argon2id;
use argon2::password_hash::SaltString;
use sea_orm::{ActiveModelTrait, ColumnTrait, Database as SeaOrmDatabase, DatabaseConnection, QueryFilter, TransactionTrait};
use async_trait::async_trait;
use lazy_static::lazy_static;
use log::error;
use rand_core::OsRng;
use regex::Regex;
use sea_orm::ActiveValue::Set;
use crate::orm_entities::figure::Entity as FigureEntity;
use crate::orm_entities::user as user;
use crate::orm_entities::user::Entity as UserEntity;
use crate::orm_entities::profile;
use crate::orm_entities::profile::Entity as ProfileEntity;
use crate::entities::figure::Figure;
use sea_orm::EntityTrait;
use serde::{Deserialize};
use unicode_segmentation::UnicodeSegmentation;
use zeroize::Zeroize;
use crate::auth_layer::hash_password;
use crate::entities::profile::{Profile, ProfileDTO};
use crate::entities::user::{User, UserDTO};
use crate::server_errors::ServerError;

#[derive(Deserialize)]
pub struct SignUpForm {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct SignInForm {
    pub email: String,
    pub password: String,
}

#[async_trait]
pub trait DatabaseFns: Sync + Send + Debug {
    async fn get_figure(&self, id: &i32) -> Option<Figure>;
    async fn signup_user(&self, signup: SignUpForm) -> Result<(UserDTO, ProfileDTO), ServerError>;
    async fn authenticate_user_by_email(&self, email: String, password: String) -> Result<(UserDTO, ProfileDTO), ServerError>;
}

pub type Database = Box<dyn DatabaseFns>;

#[derive(Debug)]
struct DatabaseImpl {
    db: DatabaseConnection,
}

#[async_trait]
impl DatabaseFns for DatabaseImpl {
    async fn get_figure(&self, id: &i32) -> Option<Figure> {
        let figure_model_option = FigureEntity::find_by_id(*id).one(&self.db).await.unwrap();
        match figure_model_option {
            Some(figure_model) => {
                Some(Figure {
                    id: figure_model.id,
                    title: figure_model.title,
                    width: figure_model.width,
                    height: figure_model.height,
                    profile_id: figure_model.profile_id,
                })
            }
            None => None
        }
    }

    async fn signup_user(&self, mut signup: SignUpForm) -> Result<(UserDTO, ProfileDTO), ServerError> {
        lazy_static! {
            static ref EMAIL_REGEX: Regex = Regex::new("^[a-zA-Z0-9_+&*-]+(?:\\.[a-zA-Z0-9_+&*-]+)*@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,}$").unwrap();
            static ref USERNAME_REGEX: Regex = Regex::new("^[a-zA-Z0-9]+-*[a-zA-Z0-9]+?$").unwrap();
        }

        // Valid email test (OWASP Regex + maximum length of 60 graphemes
        if !EMAIL_REGEX.is_match(&*signup.email) || signup.email.graphemes(true).count() > 60 {
            return Err(ServerError::InvalidEmail);
        }

        // Valid username test
        // (alphanumerical, optionally a dash surrounded by alphanumerical characters, 15 character limit)
        if !USERNAME_REGEX.is_match(&*signup.username) || signup.username.graphemes(true).count() > 15 {
            return Err(ServerError::InvalidUsername);
        }

        let password_hash_result = hash_password(&signup.password, true);
        signup.password.zeroize();
        let password_hash = match password_hash_result {
            Ok(hash) => hash,
            Err(e) => return Err(e)
        };

        // Create User and Profile in database and return these
        let transaction_result = self.db.begin().await;
        let transaction = match transaction_result {
            Ok(transaction) => transaction,
            Err(_) => {
                return Err(ServerError::InternalError);
            }
        };

        let user_model_result = user::ActiveModel {
            email: Set(signup.email.to_lowercase()),
            password: Set(password_hash),
            role: Set("user".to_string()),
            id: Default::default(),
        }.save(&transaction).await;
        let user_model = match user_model_result {
            Ok(user_model) => user_model,
            Err(_) => return Err(ServerError::EmailAlreadyInUse)
        };

        let profile_model_result = profile::ActiveModel {
            id: Default::default(),
            username: Set(signup.username.clone()),
            display_name: Default::default(),
            user_id: user_model.id.clone(),
        }.save(&transaction).await;
        let profile_model = match profile_model_result {
            Ok(profile_model) => profile_model,
            Err(_) => return Err(ServerError::UsernameAlreadyTaken)
        };

        if transaction.commit().await.is_err() {
            return Err(ServerError::InternalError);
        };

        Ok((
            UserDTO {
                email: user_model.email.unwrap(),
                role: user_model.role.unwrap(),
                id: user_model.id.unwrap(),
            },
            ProfileDTO {
                id: profile_model.id.unwrap(),
                username: profile_model.username.unwrap(),
                display_name: profile_model.display_name.unwrap(),
            }
        ))
    }

    async fn authenticate_user_by_email(&self, email: String, password: String) -> Result<(UserDTO, ProfileDTO), ServerError> {
        let user_result = UserEntity::find()
            .filter(user::Column::Email.eq(email.to_lowercase()))
            .find_also_related(ProfileEntity)
            .one(&self.db)
            .await;
        let user_option = match user_result {
            Ok(user_option) => user_option,
            Err(error) => {
                error!("{}", error);
                return Err(ServerError::InternalError);
            }
        };

        let found_user = match user_option {
            Some(found_user) => found_user,
            None => return Err(ServerError::UserWithEmailNotFound)
        };

        let parsed_hash_result = PasswordHash::new(&found_user.0.password);
        let parsed_hash = match parsed_hash_result {
            Ok(hash) => hash,
            Err(e) => {
                error!("{}", e);
                return Err(ServerError::InternalError);
            }
        };

        let password_verification = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
        if password_verification.is_ok() {
            if let Some(profile) = found_user.1 {
                Ok((
                    UserDTO {
                        email: found_user.0.email,
                        role: found_user.0.role,
                        id: found_user.0.id,
                    },
                    ProfileDTO {
                        id: profile.id,
                        username: profile.username,
                        display_name: profile.display_name,
                    }
                ))
            }
            // If no profile associated with user is found
            else {
                return Err(ServerError::InternalError);
            }
        }
        else {
            Err(ServerError::WrongPassword)
        }
    }
}

pub async fn get_database_connection(database_url: String) -> Database {
    let db: DatabaseConnection = SeaOrmDatabase::connect(database_url).await.unwrap();
    return Box::new(DatabaseImpl {
        db
    });
}