use std::sync::Arc;
use sqlx::{Error, Pool, Postgres, Row};
use crate::server_errors::ServerError;
use async_trait::async_trait;
use crate::entities::dtos::figure_dto::FigureDTO;
use crate::domain::models::figure::Figure;
use crate::infrastructure::models::figure::FigureDef;
use crate::infrastructure::models::profile::ProfileDef;
use crate::domain::models::types::IdType;
use interpol::format as iformat;
use crate::repositories::traits::{FigureRepositoryTrait, TransactionTrait};
use crate::repositories::transaction::PostgresTransaction;

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
impl FigureRepositoryTrait<PostgresTransaction> for FigureRepository {
    async fn create(&self, transaction: Option<&mut PostgresTransaction>, mut figure: Figure) -> Result<Figure, ServerError> {
        let query_string = iformat!(r#"
            INSERT INTO {FigureDef::Table}
            ({FigureDef::Id.as_str()}, {FigureDef::Title.as_str()}, {FigureDef::Description.as_str()},
            {FigureDef::Width.as_str()}, {FigureDef::Height.as_str()}, {FigureDef::Url.as_str()},
            {FigureDef::ProfileId.as_str()})
            VALUES (DEFAULT, $1, $2, $3, $4, $5, $6)
            RETURNING {FigureDef::Id.as_str()};
            "#);

        let query =
            sqlx::query(&query_string)
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
            .and_then(|row| row.try_get(0))
            .map(|id| {
                figure.id = id;
                figure
            })
            .map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }

    async fn find_by_id(&self, transaction: Option<&mut PostgresTransaction>, figure_id: IdType) -> Result<FigureDTO, ServerError> {
        let query_string = iformat!(r#"
            SELECT
            {FigureDef::Id} AS {FigureDef::Id.unique()}, {FigureDef::Title}, {FigureDef::Description},
            {FigureDef::Url}, {FigureDef::Width}, {FigureDef::Height},

            {ProfileDef::Id} AS {ProfileDef::Id.unique()}, {ProfileDef::Username}, {ProfileDef::DisplayName},
            {ProfileDef::Bio}, {ProfileDef::Banner}, {ProfileDef::ProfilePicture}, {ProfileDef::UserId}

            FROM {FigureDef::Table}
            INNER JOIN {ProfileDef::Table}
            ON {FigureDef::ProfileId} = {ProfileDef::Id}
            WHERE {FigureDef::Id} = $1
            "#);

        let query = sqlx::query_as(&query_string).bind(figure_id);

        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }.map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }

    async fn find_starting_from_id_with_profile_id(&self, transaction: Option<&mut PostgresTransaction>, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<FigureDTO>, ServerError> {
        let mut query_string = iformat!(r#"
            SELECT {FigureDef::Id} AS {FigureDef::Id.unique()}, {FigureDef::Title}, {FigureDef::Description}, {FigureDef::Url}, {FigureDef::Width}, {FigureDef::Height},
            {ProfileDef::Id} AS {ProfileDef::Id.unique()}, {ProfileDef::Username}, {ProfileDef::DisplayName}, {ProfileDef::Bio}, {ProfileDef::Banner}, {ProfileDef::ProfilePicture}, {ProfileDef::UserId}
            FROM {FigureDef::Table}
            INNER JOIN {ProfileDef::Table}
            ON {FigureDef::ProfileId} = {ProfileDef::Id}
            "#);

        // Filter figures by starting from figure id.
        if let Some(starting_from_id) = figure_id {
            query_string = iformat!(r#"
            {query_string}
            WHERE {FigureDef::Id} < {starting_from_id}
            "#);
        }

        // Filter by profile
        if let Some(from_profile) = profile_id {
            let mut filter = iformat!("{FigureDef::ProfileId} = {from_profile}");
            // Check if where clause already exists in query (only figure_id will add where clause)
            if figure_id.is_some() {
                filter = iformat!("AND {filter}");
            } else {
                filter = iformat!("WHERE {filter}");
            }
            query_string = iformat!(r#"
            {query_string}
            {filter}
            "#);
        }

        query_string = iformat!(r#"
        {query_string}
        ORDER BY {FigureDef::Id} DESC
        LIMIT {limit}
        "#);

        let query = sqlx::query_as::<_, FigureDTO>(&query_string);

        match transaction {
            Some(transaction) => query.fetch_all(transaction.inner()).await,
            None => query.fetch_all(&self.db).await
        }.map_err(|e| match e {
            Error::RowNotFound => ServerError::ResourceNotFound,
            e => ServerError::InternalError(Arc::new(e.into()))
        })
    }

    async fn update_figure(&self, transaction: Option<&mut PostgresTransaction>, figure: Figure) -> Result<(), ServerError> {
        let query_string = iformat!(r#"
            UPDATE {FigureDef::Table}
            SET {FigureDef::Title} = $2, {FigureDef::Description} = $3, {FigureDef::Url} = $4, {FigureDef::Width} = $5, {FigureDef::Height} = $6
            WHERE {FigureDef::Id} = $1
            "#);

        let query =
            sqlx::query(&query_string)
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
            .map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }

    async fn delete_figure_by_id(&self, transaction: Option<&mut PostgresTransaction>, figure_id: IdType) -> Result<(), ServerError> {
        let query_string = iformat!(r#"
            DELETE FROM {FigureDef::Table}
            WHERE {FigureDef::Id} = $1
            "#);
        let query =
            sqlx::query(&query_string)
                .bind(figure_id);
        match transaction {
            Some(transaction) => query.execute(transaction.inner()).await,
            None => query.execute(&self.db).await
        }
            .map(|_result| ())
            .map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }

    async fn count_by_profile_id(&self, transaction: Option<&mut PostgresTransaction>, profile_id: IdType) -> Result<IdType, ServerError> {
        let query_string = iformat!(r#"
        SELECT count(*) FROM {FigureDef::Table}
        where {FigureDef::ProfileId} = $1
        "#);
        let query =
            sqlx::query(&query_string)
                .bind(profile_id);
        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .and_then(|row| row.try_get(0))
            .map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }

    async fn get_total_figures_count(&self, transaction: Option<&mut PostgresTransaction>) -> Result<IdType, ServerError> {
        let query_string = iformat!(r#"
        SELECT count(*) FROM {FigureDef::Table}
        "#);
        let query =
            sqlx::query(&query_string);
        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .and_then(|row| row.try_get(0))
            .map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }
}