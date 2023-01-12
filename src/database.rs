use std::fmt::{Debug};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use crate::entities::figure::{Figure};
use crate::entities::user::{UserAndProfileFromQuery};
use serde::{Deserialize};
use sqlx::{Error, PgPool, Pool, Postgres, Row};
use zeroize::Zeroize;
use crate::auth_layer::{hash_password, is_email_valid, is_username_valid};
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::dtos::user_dto::UserDTO;
use crate::entities::profile::ProfileDef;
use crate::server_errors::ServerError;
use crate::entities::types::{Id, IdType};

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
    async fn get_figure(&self, id: &IdType) -> Result<Figure, ServerError<String>>;
    async fn get_profile_dto_by_id(&self, id: IdType) -> Result<ProfileDTO, ServerError<String>>;
    async fn signup_user(&self, signup: SignUpForm) -> Result<(UserDTO, ProfileDTO), ServerError<String>>;
    async fn authenticate_user_by_email(&self, email: String, password: String) -> Result<(UserDTO, ProfileDTO), ServerError<String>>;
}

pub type Database = Box<dyn DatabaseFns>;

#[derive(Debug)]
struct DatabaseImpl {
    db: Pool<Postgres>,
}

#[async_trait]
impl DatabaseFns for DatabaseImpl {
    async fn get_figure(&self, id: &IdType) -> Result<Figure, ServerError<String>> {
        let query =
            sqlx::query_as::<_, Figure>(r#"
            SELECT figures.id, figures.title, figures.description, figures.url, figures.width, figures.height, figures.profile_id,
            profiles.username, profiles.display_name
            from figures
            INNER JOIN profiles
            ON figures.id = profiles.id
            where figures.id = $1
            "#)
                .bind(id)
                .fetch_one(&self.db).await;
        match query {
            Ok(figure) => Ok(figure),
            Err(Error::RowNotFound) => Err(ServerError::ResourceNotFound),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn get_profile_dto_by_id(&self, id: IdType) -> Result<ProfileDTO, ServerError<String>> {
        let query =
            sqlx::query_as::<_, ProfileDTO>(&format!("SELECT id, username, display_name from {} where {} = $1", ProfileDef::Table, ProfileDef::Id))
                .bind(id)
                .fetch_one(&self.db).await;
        match query {
            Ok(profile) => Ok(profile),
            Err(Error::RowNotFound) => Err(ServerError::ResourceNotFound),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }


    async fn signup_user(&self, mut signup: SignUpForm) -> Result<(UserDTO, ProfileDTO), ServerError<String>> {
        if !is_email_valid(&signup.email) {
            return Err(ServerError::InvalidEmail);
        }
        if !is_username_valid(&signup.username) {
            return Err(ServerError::InvalidUsername);
        }
        let password_hash_result = hash_password(&signup.password, true);
        signup.password.zeroize();
        let password_hash = match password_hash_result {
            Ok(hash) => hash,
            Err(e) => return Err(e)
        };

        let transaction_result = self.db.begin().await;
        let mut transaction = match transaction_result {
            Ok(transaction) => transaction,
            Err(e) => {
                return Err(ServerError::InternalError(e.to_string()));
            }
        };

        // Create a user
        let user_id_result = sqlx::query_as::<_, Id>(r#"
            INSERT INTO users (email, password, role)
            VALUES ($1, $2, 'user')
            RETURNING id"#)
            .bind(&signup.email.to_lowercase())
            .bind(password_hash)
            .fetch_one(&mut transaction).await;

        let user_id = match user_id_result {
            Ok(user_id) => user_id,
            Err(e) => {
                return match ServerError::parse_db_error(&e) {
                    ServerError::ConstraintError => {
                        Err(ServerError::EmailAlreadyInUse)
                    }
                    _ => Err(ServerError::InternalError(e.to_string()))
                };
            }
        };

        // Create a profile
        let profile_id_result = sqlx::query_as::<_, Id>(r#"
            INSERT INTO profiles (username, user_id)
            VALUES ($1, $2)
            RETURNING id"#)
            .bind(&signup.username)
            .bind(user_id.0)
            .fetch_one(&mut transaction).await;

        let profile_id = match profile_id_result {
            Ok(profile_id) => profile_id,
            Err(e) => {
                return match ServerError::parse_db_error(&e) {
                    ServerError::ConstraintError => {
                        Err(ServerError::UsernameAlreadyTaken)
                    }
                    _ => Err(ServerError::InternalError(e.to_string()))
                };
            }
        };

        match transaction.commit().await {
            Ok(()) => Ok((
                UserDTO {
                    email: signup.email,
                    role: "user".to_string(),
                    id: user_id,
                },
                ProfileDTO {
                    id: profile_id,
                    username: signup.username,
                    display_name: None,
                }
            )),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn authenticate_user_by_email(&self, email: String, password: String) -> Result<(UserDTO, ProfileDTO), ServerError<String>> {
        let user_profile_result = sqlx::query_as::<_, UserAndProfileFromQuery>(r#"
        SELECT profiles.id AS profile_id, users.id AS user_id, users.email, users.password, users.role, profiles.username, profiles.display_name
        FROM users
        INNER JOIN profiles
        ON users.id = profiles.user_id
        WHERE users.email = $1
        "#)
            .bind(email)
            .fetch_one(&self.db).await;
        let (user, profile) = match user_profile_result {
            Ok(user_and_profile) => (
                user_and_profile.user,
                user_and_profile.profile
            ),
            Err(Error::RowNotFound) => return Err(ServerError::ResourceNotFound),
            Err(e) => return Err(ServerError::InternalError(e.to_string()))
        };

        let parsed_hash_result = PasswordHash::new(&user.password);
        let parsed_hash = match parsed_hash_result {
            Ok(hash) => hash,
            Err(e) => {
                return Err(ServerError::InternalError(e.to_string()));
            }
        };

        let password_verification = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
        if password_verification.is_ok() {
            Ok((
                user.into(), profile.into()
            ))
        } else {
            Err(ServerError::WrongPassword)
        }
    }
}

pub async fn get_database_connection(database_url: String) -> Database {
    let db = PgPool::connect(&database_url).await.unwrap();
    Box::new(DatabaseImpl {
        db
    })
}