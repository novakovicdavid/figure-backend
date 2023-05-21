use dyn_clone::DynClone;
use sqlx::{Error, Pool, Postgres, Transaction};
use crate::server_errors::ServerError;
use async_trait::async_trait;
use crate::entities::dtos::figure_dto::FigureDTO;
use crate::entities::figure::Figure;
use crate::entities::profile::Profile;
use crate::entities::types::IdType;

#[derive(Clone)]
pub struct FigureRepository {
    db: Pool<Postgres>,
}

impl FigureRepository {
    pub fn new(pool: Pool<Postgres>) -> impl FigureRepositoryTrait {
        FigureRepository {
            db: pool
        }
    }
}

#[async_trait]
pub trait FigureRepositoryTrait: Send + Sync + DynClone {
    async fn start_transaction(&self) -> Result<Transaction<Postgres>, ServerError<String>>;
    async fn create(&self, transaction: Option<&mut Transaction<Postgres>>, figure: Figure) -> Result<Figure, ServerError<String>>;
    async fn find_by_id(&self, transaction: Option<&mut Transaction<Postgres>>, figure_id: IdType) -> Result<FigureDTO, ServerError<String>>;
    // async fn find_by_profile_id(&self, transaction: Option<&mut Transaction<Postgres>>, profile_id: IdType) -> Result<Figure, ServerError<String>>;
    async fn update_figure(&self, transaction: Option<&mut Transaction<Postgres>>, figure: Figure) -> Result<(), ServerError<String>>;
    async fn delete_figure_by_id(&self, transaction: Option<&mut Transaction<Postgres>>, figure_id: IdType) -> Result<(), ServerError<String>>;
}

#[async_trait]
impl FigureRepositoryTrait for FigureRepository {
    async fn start_transaction(&self) -> Result<Transaction<Postgres>, ServerError<String>> {
        match self.db.begin().await {
            Ok(transaction) => Ok(transaction),
            Err(e) => Err(ServerError::TransactionFailed)
        }
    }

    async fn create(&self, transaction: Option<&mut Transaction<Postgres>>, figure: Figure) -> Result<Figure, ServerError<String>> {
        let query =
            sqlx::query_as(r#"
            INSERT INTO figures (id, title, description, width, height, url, profile_id)
            VALUES (DEFAULT, $1, $2, $3, $4, $5, $6)
            RETURNING id;
            "#)
                .bind(figure.title)
                .bind(figure.description)
                .bind(figure.width)
                .bind(figure.height)
                .bind(figure.url)
                .bind(figure.profile_id);
        match transaction {
            Some(transaction) => query.fetch_one(transaction).await,
            None => query.fetch_one(&self.db).await
        }.map_err(|e| ServerError::InternalError(e.to_string()))
    }

    async fn find_by_id(&self, transaction: Option<&mut Transaction<Postgres>>, figure_id: IdType) -> Result<FigureDTO, ServerError<String>> {
        let query =
            sqlx::query_as(r#"
            SELECT figures.id AS figure_id, figures.title, figures.description, figures.url, figures.width, figures.height,
            profiles.id AS profile_id, profiles.username, profiles.display_name, profiles.bio, profiles.banner, profiles.profile_picture, profiles.user_id
            FROM figures
            INNER JOIN profiles
            ON figures.profile_id = profiles.id
            WHERE figures.id = $1
            "#)
                .bind(figure_id);
        match transaction {
            Some(transaction) => query.fetch_one(transaction).await,
            None => query.fetch_one(&self.db).await
        }.map_err(|e| ServerError::InternalError(e.to_string()))
    }

    // async fn find_by_profile_id(&self, transaction: Option<&mut Transaction<Postgres>>, profile_id: IdType) -> Result<FigureDTO, ServerError<String>> {
    //     let query =
    //         sqlx::query_as(r#"
    //         SELECT figures.id AS figure_id, figures.title, figures.description, figures.url, figures.width, figures.height,
    //         profiles.id AS profile_id, profiles.username, profiles.display_name, profiles.bio, profiles.banner, profiles.profile_picture, profiles.user_id
    //         FROM figures
    //         INNER JOIN profiles
    //         ON figures.profile_id = profiles.id
    //         where profiles.id = $1
    //         "#)
    //             .bind(profile_id);
    //     match transaction {
    //         Some(transaction) => query.fetch_one(transaction).await,
    //         None => query.fetch_one(&self.db).await
    //     }.map_err(|e| ServerError::InternalError(e.to_string()))
    // }

    async fn update_figure(&self, transaction: Option<&mut Transaction<Postgres>>, figure: Figure) -> Result<(), ServerError<String>> {
        let query =
            sqlx::query(r#"
            UPDATE figures
            SET figures.title = $2, figures.description = $3, figures.url = $4, figures.width = $5, figures.height = $6
            WHERE figures.id = $1
            "#)
                .bind(figure.id)
                .bind(figure.title)
                .bind(figure.description)
                .bind(figure.url)
                .bind(figure.width)
                .bind(figure.height);
        match transaction {
            Some(transaction) => query.execute(transaction).await,
            None => query.execute(&self.db).await
        }
            .map(|_result| ())
            .map_err(|e| ServerError::InternalError(e.to_string()))
    }

    async fn delete_figure_by_id(&self, transaction: Option<&mut Transaction<Postgres>>, figure_id: IdType) -> Result<(), ServerError<String>> {
        let query =
            sqlx::query(r#"
            DELETE FROM figures
            WHERE figures.id = $1
            "#)
                .bind(figure_id);
        match transaction {
            Some(transaction) => query.execute(transaction).await,
            None => query.execute(&self.db).await
        }
            .map(|_result| ())
            .map_err(|e| ServerError::InternalError(e.to_string()))
    }
}