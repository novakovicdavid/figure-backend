use sqlx::{Error, Pool, Postgres, Row, Transaction};
use crate::server_errors::ServerError;
use async_trait::async_trait;
use crate::entities::dtos::figure_dto::FigureDTO;
use crate::entities::figure::Figure;
use crate::entities::types::IdType;
use crate::MyTransaction;
use crate::repositories::transaction::{PostgresTransaction, TransactionTrait};

#[derive(Clone)]
pub struct FigureRepository {
    db: Pool<Postgres>,
}

impl FigureRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        FigureRepository {
            db: pool
        }
    }
}

#[async_trait]
pub trait FigureRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, figure: Figure) -> Result<Figure, ServerError<String>>;
    async fn find_by_id(&self, transaction: Option<&mut MyTransaction>, figure_id: IdType) -> Result<FigureDTO, ServerError<String>>;
    // async fn find_by_profile_id(&self, transaction: Option<&mut Transaction<Postgres>>, profile_id: IdType) -> Result<Figure, ServerError<String>>;
    async fn find_starting_from_id_with_profile_id(&self, transaction: Option<&mut MyTransaction>, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<FigureDTO>, ServerError<String>>;
    async fn update_figure(&self, transaction: Option<&mut MyTransaction>, figure: Figure) -> Result<(), ServerError<String>>;
    async fn delete_figure_by_id(&self, transaction: Option<&mut MyTransaction>, figure_id: IdType) -> Result<(), ServerError<String>>;
}

#[async_trait]
impl FigureRepositoryTrait<PostgresTransaction> for FigureRepository {
    async fn create(&self, transaction: Option<&mut MyTransaction>, mut figure: Figure) -> Result<Figure, ServerError<String>> {
        let query =
            sqlx::query(r#"
            INSERT INTO figures (id, title, description, width, height, url, profile_id)
            VALUES (DEFAULT, $1, $2, $3, $4, $5, $6)
            RETURNING id;
            "#)
                .bind(figure.title.clone())
                .bind(figure.description.clone())
                .bind(figure.width)
                .bind(figure.height)
                .bind(figure.url.clone())
                .bind(figure.profile_id);
        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .map(|row| {
                figure.id = row.get(0);
                figure
            })
            .map_err(|e| ServerError::InternalError(e.to_string()))
    }

    async fn find_by_id(&self, transaction: Option<&mut MyTransaction>, figure_id: IdType) -> Result<FigureDTO, ServerError<String>> {
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
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }.map_err(|e| ServerError::InternalError(e.to_string()))
    }

    async fn find_starting_from_id_with_profile_id(&self, transaction: Option<&mut MyTransaction>, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<FigureDTO>, ServerError<String>> {
        let mut query = r#"
            SELECT figures.id AS figure_id, figures.title, figures.description, figures.url, figures.width, figures.height,
            profiles.id AS profile_id, profiles.username, profiles.display_name, profiles.bio, profiles.banner, profiles.profile_picture, profiles.user_id
            from figures
            INNER JOIN profiles
            ON figures.profile_id = profiles.id
            "#.to_string();

        if let Some(starting_from_id) = figure_id {
            query = format!(r#"
            {}
            WHERE figures.id < {}
            "#, query, starting_from_id);
        }

        if let Some(from_profile) = profile_id {
            let mut filter = "figures.profile_id = ".to_string();
            if figure_id.is_some() {
                filter = format!("AND {}", filter);
            } else {
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

        let query = sqlx::query_as::<_, FigureDTO>(&query);
        let result = match transaction {
            Some(transaction) => query.fetch_all(transaction.inner()).await,
            None => query.fetch_all(&self.db).await
        };

        match result {
            Ok(figures) => Ok(figures),
            Err(Error::RowNotFound) => Err(ServerError::ResourceNotFound),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
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

    async fn update_figure(&self, transaction: Option<&mut MyTransaction>, figure: Figure) -> Result<(), ServerError<String>> {
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
            Some(transaction) => query.execute(transaction.inner()).await,
            None => query.execute(&self.db).await
        }
            .map(|_result| ())
            .map_err(|e| ServerError::InternalError(e.to_string()))
    }

    async fn delete_figure_by_id(&self, transaction: Option<&mut MyTransaction>, figure_id: IdType) -> Result<(), ServerError<String>> {
        let query =
            sqlx::query(r#"
            DELETE FROM figures
            WHERE figures.id = $1
            "#)
                .bind(figure_id);
        match transaction {
            Some(transaction) => query.execute(transaction.inner()).await,
            None => query.execute(&self.db).await
        }
            .map(|_result| ())
            .map_err(|e| ServerError::InternalError(e.to_string()))
    }
}