use std::fmt::Debug;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use crate::entities::figure::Figure;
use crate::entities::user::{User, UserAndProfileFromQuery};
use serde::Deserialize;
use sqlx::{Error, FromRow, PgPool, Pool, Postgres, Row};
use crate::entities::dtos::figure_dto::FigureDTO;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::profile::Profile;
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
    async fn get_total_profiles_count(&self) -> Result<IdType, ServerError<String>>;
    async fn get_total_figures_count(&self) -> Result<IdType, ServerError<String>>;
    async fn get_figure(&self, id: &IdType) -> Result<FigureDTO, ServerError<String>>;
    // async fn get_figures(&self, starting_from_id: Option<IdType>, from_profile: Option<IdType>, limit: &IdType) -> Result<Vec<FigureDTO>, ServerError<String>>;
    async fn create_figure(&self, title: String, description: String, width: i32, height: i32, url: String, profile_id: IdType) -> Result<IdType, ServerError<String>>;
    async fn get_total_figures_by_profile(&self, profile_id: IdType) -> Result<IdType, ServerError<String>>;
}

pub type Database = Box<dyn DatabaseFns>;

#[derive(Debug)]
struct DatabaseImpl {
    db: Pool<Postgres>,
}

#[async_trait]
impl DatabaseFns for DatabaseImpl {
    async fn get_total_profiles_count(&self) -> Result<IdType, ServerError<String>> {
        let query =
            sqlx::query(r#"
            SELECT count(*) AS count FROM profiles;
            "#)
                .fetch_one(&self.db).await;
        match query {
            Ok(count) => {
                match count.try_get(0) {
                    Ok(count) => {
                        Ok(count)
                    }
                    Err(e) => Err(ServerError::InternalError(e.to_string()))
                }
            }
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
            Ok(count) => {
                match count.try_get(0) {
                    Ok(count) => {
                        Ok(count)
                    }
                    Err(e) => Err(ServerError::InternalError(e.to_string()))
                }
            }
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn get_figure(&self, id: &IdType) -> Result<FigureDTO, ServerError<String>> {
        let query =
            sqlx::query(r#"
            SELECT figures.id AS figure_id, figures.title, figures.description, figures.url, figures.width, figures.height,
            profiles.id AS profile_id, profiles.username, profiles.display_name, profiles.bio, profiles.banner, profiles.profile_picture, profiles.user_id
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
            }
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

    // async fn get_figures(&self, starting_from_id: Option<IdType>, from_profile: Option<IdType>, limit: &IdType) -> Result<Vec<FigureDTO>, ServerError<String>> {
    //     let mut query = r#"
    //         SELECT figures.id AS figure_id, figures.title, figures.description, figures.url, figures.width, figures.height,
    //         profiles.id AS profile_id, profiles.username, profiles.display_name, profiles.bio, profiles.banner, profiles.profile_picture, profiles.user_id
    //         from figures
    //         INNER JOIN profiles
    //         ON figures.profile_id = profiles.id
    //         "#.to_string();
    //
    //     if let Some(starting_from_id) = starting_from_id {
    //         query = format!(r#"
    //         {}
    //         WHERE figures.id < {}
    //         "#, query, starting_from_id);
    //     }
    //
    //     if let Some(from_profile) = from_profile {
    //         let mut filter = "figures.profile_id = ".to_string();
    //         if starting_from_id.is_some() {
    //             filter = format!("AND {}", filter);
    //         } else {
    //             filter = format!("WHERE {}", filter);
    //         }
    //         query = format!(r#"
    //         {}
    //         {} {}
    //         "#, query, filter, from_profile);
    //     }
    //
    //     query = format!(r#"
    //     {}
    //     ORDER BY figures.id DESC
    //     LIMIT {}
    //     "#, query, limit);
    //
    //     let query = sqlx::query_as::<_, FigureDTO>(&query);
    //
    //     let result = sqlx::query_as::<_, FigureDTO>(&query).fetch_all(&self.db).await;
    //     match result {
    //         Ok(figures) => Ok(figures),
    //         Err(Error::RowNotFound) => Err(ServerError::ResourceNotFound),
    //         Err(e) => Err(ServerError::InternalError(e.to_string()))
    //     }
    // }

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
            }
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn get_total_figures_by_profile(&self, profile_id: IdType) -> Result<IdType, ServerError<String>> {
        let result =
            sqlx::query(r#"
            SELECT COUNT(*)
            FROM figures
            WHERE profile_id = $1
            "#)
                .bind(profile_id)
                .fetch_one(&self.db).await;
        match result {
            Ok(total) => {
                match total.try_get(0) {
                    Ok(id) => Ok(id),
                    Err(e) => Err(ServerError::InternalError(e.to_string()))
                }
            }
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }
}

pub async fn get_database_connection(database_url: &str) -> impl DatabaseFns {
    let db = PgPool::connect(database_url).await.unwrap();
    DatabaseImpl {
        db
    }
}