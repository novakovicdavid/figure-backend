use std::fmt::{Debug};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use crate::entities::figure::{Figure};
use crate::entities::user::{User, UserAndProfileFromQuery};
use serde::{Deserialize};
use sqlx::{Error, FromRow, PgPool, Pool, Postgres, Row};
use zeroize::Zeroize;
use crate::auth_layer::{hash_password, is_email_valid, is_username_valid};
use crate::entities::dtos::figure_dto::FigureDTO;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::dtos::user_dto::UserDTO;
use crate::entities::profile::{Profile, ProfileDef};
use crate::server_errors::ServerError;
use crate::entities::types::IdType;

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
    async fn signup_user(&self, signup: SignUpForm) -> Result<(User, Profile), ServerError<String>>;
    async fn authenticate_user_by_email(&self, email: String, password: String) -> Result<(User, Profile), ServerError<String>>;
    async fn get_profile_by_id(&self, id: IdType) -> Result<Profile, ServerError<String>>;
    async fn get_total_profiles_count(&self) -> Result<IdType, ServerError<String>>;
    async fn get_total_figures_count(&self) -> Result<IdType, ServerError<String>>;
    async fn get_figure(&self, id: &IdType) -> Result<FigureDTO, ServerError<String>>;
    async fn get_figures(&self, starting_from_id: Option<IdType>, from_profile: Option<IdType>, limit: &IdType) -> Result<Vec<FigureDTO>, ServerError<String>>;
    async fn create_figure(&self, title: String, description: String, width: i32, height: i32, url: String, profile_id: IdType) -> Result<IdType, ServerError<String>>;
}

pub type Database = Box<dyn DatabaseFns>;

#[derive(Debug)]
struct DatabaseImpl {
    db: Pool<Postgres>,
}

#[async_trait]
impl DatabaseFns for DatabaseImpl {
    async fn signup_user(&self, mut signup: SignUpForm) -> Result<(User, Profile), ServerError<String>> {
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
        let user_id_result = sqlx::query(r#"
            INSERT INTO users (email, password, role)
            VALUES ($1, $2, 'user')
            RETURNING id"#)
            .bind(&signup.email.to_lowercase())
            .bind(password_hash)
            .fetch_one(&mut transaction).await;

        let user_id = match user_id_result {
            Ok(user_id) => user_id.get(0),
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
        let profile_id_result = sqlx::query(r#"
            INSERT INTO profiles (username, user_id)
            VALUES ($1, $2)
            RETURNING id"#)
            .bind(&signup.username)
            .bind(user_id)
            .fetch_one(&mut transaction).await;

        let profile_id = match profile_id_result {
            Ok(profile_id) => profile_id.get(0),
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
                User {
                    email: signup.email,
                    password: "".to_string(),
                    role: "user".to_string(),
                    id: user_id,
                },
                Profile {
                    id: profile_id,
                    username: signup.username,
                    display_name: None,
                    user_id,
                }
            )),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn authenticate_user_by_email(&self, email: String, password: String) -> Result<(User, Profile), ServerError<String>> {
        let user_profile_result = sqlx::query_as::<_, UserAndProfileFromQuery>(r#"
        SELECT users.id AS user_id, users.email, users.password, users.role,
        profiles.id AS profile_id, profiles.username, profiles.display_name
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
            Err(Error::RowNotFound) => return Err(ServerError::UserWithEmailNotFound),
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


    async fn get_profile_by_id(&self, id: IdType) -> Result<Profile, ServerError<String>> {
        let query =
            sqlx::query_as::<_, Profile>(&format!("SELECT id, username, display_name, user_id from {} where {} = $1", ProfileDef::Table, ProfileDef::Id))
                .bind(id)
                .fetch_one(&self.db).await;
        match query {
            Ok(profile) => Ok(profile),
            Err(Error::RowNotFound) => Err(ServerError::ResourceNotFound),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn get_total_profiles_count(&self) -> Result<IdType, ServerError<String>> {
        let query =
            sqlx::query(r#"
            SELECT reltuples AS count FROM pg_class where relname = 'profiles';
            "#)
                .fetch_one(&self.db).await;
        match query {
            Ok(id) => {
                match id.try_get::<f32, _>(0) {
                    Ok(id) => Ok(id as IdType),
                    Err(e) => Err(ServerError::InternalError(e.to_string()))
                }
            },
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn get_total_figures_count(&self) -> Result<IdType, ServerError<String>> {
        let query =
            sqlx::query(r#"
            SELECT count(*) AS count FROM figures;
            "#)
                .fetch_one(&self.db).await;
        match query {
            Ok(id) => {
                match id.try_get(0) {
                    Ok(id) => {
                        Ok(id)
                    },
                    Err(e) => Err(ServerError::InternalError(e.to_string()))
                }
            },
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn get_figure(&self, id: &IdType) -> Result<FigureDTO, ServerError<String>> {
        let query =
            sqlx::query(r#"
            SELECT figures.id AS figure_id, figures.title, figures.description, figures.url, figures.width, figures.height,
            profiles.id AS profile_id, profiles.username, profiles.display_name, profiles.user_id
            from figures
            INNER JOIN profiles
            ON figures.profile_id = profiles.id
            where figures.id = $1
            "#)
                .bind(id)
                .fetch_one(&self.db).await;
        let row = match query {
            Ok(row) => {
                row
            },
            Err(Error::RowNotFound) => return Err(ServerError::ResourceNotFound),
            Err(e) => return Err(ServerError::InternalError(e.to_string()))
        };

        let figure = Figure::from_row(&row);
        let profile = Profile::from_row(&row);
        let figure_profile = (figure, profile);

        match figure_profile {
            (Ok(figure), Ok(profile)) =>
                Ok(FigureDTO::from(figure, ProfileDTO::from(profile))),
            (_, _) => return Err(ServerError::InternalError("Could not get figure or profile from row".to_string()))
        }
    }

    async fn get_figures(&self, starting_from_id: Option<IdType>, from_profile: Option<IdType>, limit: &IdType) -> Result<Vec<FigureDTO>, ServerError<String>> {
        let mut query = r#"
            SELECT figures.id AS figure_id, figures.title, figures.description, figures.url, figures.width, figures.height,
            profiles.id AS profile_id, profiles.username, profiles.display_name, profiles.user_id
            from figures
            INNER JOIN profiles
            ON figures.profile_id = profiles.id
            "#.to_string();

        if let Some(starting_from_id) = starting_from_id {
            query = format!(r#"
            {}
            WHERE figures.id < {}
            "#, query, starting_from_id);
        }

        if let Some(from_profile) = from_profile {
            let mut filter = "figures.profile_id = ".to_string();
            if starting_from_id.is_some() {
                filter = format!("AND {}", filter);
            }
            else {
                filter = format!("WHERE {}", filter);
            }
            query = format!(r#"
            {}
            {} {}
            "#, query, filter, from_profile);
        }

        query = format!(r#"
        {}
        ORDER BY figures.id DESC
        LIMIT {}
        "#, query, limit);

        println!("{}", query);

        let result = sqlx::query_as::<_, FigureDTO>(&query).fetch_all(&self.db).await;
        match result {
            Ok(figures) => Ok(figures),
            Err(Error::RowNotFound) => Err(ServerError::ResourceNotFound),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn create_figure(&self, title: String, description: String, width: i32, height: i32, url: String, profile_id: IdType) -> Result<IdType, ServerError<String>> {
        let result =
            sqlx::query(r#"
            INSERT INTO figures (id, title, description, width, height, url, profile_id)
            VALUES (DEFAULT, $1, $2, $3, $4, $5, $6)
            RETURNING id;
            "#)
                .bind(title)
                .bind(description)
                .bind(width)
                .bind(height)
                .bind(url)
                .bind(profile_id)
                .fetch_one(&self.db).await;
        match result {
            Ok(id) => {
                match id.try_get(0) {
                    Ok(id) => Ok(id),
                    Err(e) => Err(ServerError::InternalError(e.to_string()))
                }
            },
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }
}

pub async fn get_database_connection(database_url: &str) -> Database {
    let db = PgPool::connect(database_url).await.unwrap();
    Box::new(DatabaseImpl {
        db
    })
}