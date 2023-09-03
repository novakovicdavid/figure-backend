use std::sync::Arc;
use sqlx::{Error, Pool, Postgres, Row};
use crate::server_errors::ServerError;
use async_trait::async_trait;
use crate::domain::models::figure::Figure;
use crate::infrastructure::models::figure::FigureDef;
use crate::infrastructure::models::profile::ProfileDef;
use crate::domain::models::types::IdType;
use interpol::format as iformat;
use crate::domain::models::profile::Profile;
use crate::repositories::entities::FigureAndProfile;
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
            INSERT INTO {FigureDef::TABLE}
            ({FigureDef::ID}, {FigureDef::TITLE}, {FigureDef::DESCRIPTION},
            {FigureDef::WIDTH}, {FigureDef::HEIGHT}, {FigureDef::URL},
            {FigureDef::PROFILE_ID})
            VALUES (DEFAULT, $1, $2, $3, $4, $5, $6)
            RETURNING {FigureDef::ID};
            "#);

        let query =
            sqlx::query(&query_string)
                .bind(figure.get_title())
                .bind(figure.get_description())
                .bind(figure.get_width())
                .bind(figure.get_height())
                .bind(figure.get_url())
                .bind(figure.get_profile_id());

        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .and_then(|row| row.try_get(0))
            .map(|id| {
                figure.set_id(id);
                figure
            })
            .map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }

    async fn find_by_id(&self, transaction: Option<&mut PostgresTransaction>, figure_id: IdType) -> Result<(Figure, Profile), ServerError> {
        let query_string = iformat!(r#"
            SELECT
            {FigureDef::ID} AS {FigureDef::ID_UNIQUE}, {FigureDef::TITLE}, {FigureDef::DESCRIPTION},
            {FigureDef::URL}, {FigureDef::WIDTH}, {FigureDef::HEIGHT},

            {ProfileDef::ID} AS {ProfileDef::ID_UNIQUE}, {ProfileDef::USERNAME}, {ProfileDef::DISPLAY_NAME},
            {ProfileDef::BIO}, {ProfileDef::BANNER}, {ProfileDef::PROFILE_PICTURE}, {ProfileDef::USER_ID}

            FROM {FigureDef::TABLE}
            INNER JOIN {ProfileDef::TABLE}
            ON {FigureDef::PROFILE_ID} = {ProfileDef::ID_UNIQUE}
            WHERE {FigureDef::ID_UNIQUE} = $1
            "#);

        let query = sqlx::query_as::<_, FigureAndProfile>(&query_string)
            .bind(figure_id);

        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .map(|figure_and_profile| figure_and_profile.get_figure_and_profile())
            .map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }

    async fn find_starting_from_id_with_profile_id(&self, transaction: Option<&mut PostgresTransaction>, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<(Figure, Profile)>, ServerError> {
        let mut query_string = iformat!(r#"
            SELECT {FigureDef::ID} AS {FigureDef::ID_UNIQUE}, {FigureDef::TITLE}, {FigureDef::DESCRIPTION}, {FigureDef::URL}, {FigureDef::WIDTH}, {FigureDef::HEIGHT}, {FigureDef::PROFILE_ID},
            {ProfileDef::ID} AS {ProfileDef::ID_UNIQUE}, {ProfileDef::USERNAME}, {ProfileDef::DISPLAY_NAME}, {ProfileDef::BIO}, {ProfileDef::BANNER}, {ProfileDef::PROFILE_PICTURE}, {ProfileDef::USER_ID}
            FROM {FigureDef::TABLE}
            INNER JOIN {ProfileDef::TABLE}
            ON {FigureDef::PROFILE_ID} = {ProfileDef::ID}
            "#);

        // Select figures with id starting after given figure id
        if let Some(starting_from_id) = figure_id {
            query_string = iformat!(r#"
            {query_string}
            WHERE {FigureDef::ID} < {starting_from_id}
            "#);
        }

        // Filter by given profile id
        if let Some(from_profile) = profile_id {
            let mut filter = iformat!("{FigureDef::PROFILE_ID} = {from_profile}");
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

        // Order and limit
        query_string = iformat!(r#"
        {query_string}
        ORDER BY {FigureDef::ID} DESC
        LIMIT {limit}
        "#);

        let query = sqlx::query_as::<_, FigureAndProfile>(&query_string);

        match transaction {
            Some(transaction) => query.fetch_all(transaction.inner()).await,
            None => query.fetch_all(&self.db).await
        }
            .map(|figures_and_profiles| {
                figures_and_profiles
                    .into_iter()
                    .map(|figure_and_profile| figure_and_profile.get_figure_and_profile())
                    .collect()
            })
            .map_err(|e| match e {
            Error::RowNotFound => ServerError::ResourceNotFound,
            e => ServerError::InternalError(Arc::new(e.into()))
        })
    }

    async fn update_figure(&self, transaction: Option<&mut PostgresTransaction>, figure: Figure) -> Result<(), ServerError> {
        let query_string = iformat!(r#"
            UPDATE {FigureDef::TABLE}
            SET {FigureDef::TITLE} = $2, {FigureDef::DESCRIPTION} = $3, {FigureDef::URL} = $4, {FigureDef::WIDTH} = $5, {FigureDef::HEIGHT} = $6
            WHERE {FigureDef::ID} = $1
            "#);

        let query =
            sqlx::query(&query_string)
                .bind(figure.get_id())
                .bind(figure.get_title())
                .bind(figure.get_description())
                .bind(figure.get_url())
                .bind(figure.get_width())
                .bind(figure.get_height());

        match transaction {
            Some(transaction) => query.execute(transaction.inner()).await,
            None => query.execute(&self.db).await
        }
            .map(|_result| ())
            .map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }

    async fn delete_figure_by_id(&self, transaction: Option<&mut PostgresTransaction>, figure_id: IdType) -> Result<(), ServerError> {
        let query_string = iformat!(r#"
            DELETE FROM {FigureDef::TABLE}
            WHERE {FigureDef::ID} = $1
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
        SELECT count(*) FROM {FigureDef::TABLE}
        where {FigureDef::PROFILE_ID} = $1
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
        SELECT count(*) FROM {FigureDef::TABLE}
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