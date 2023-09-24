use sqlx::{Error, FromRow, Pool, Postgres, Row};
use crate::server_errors::ServerError;
use async_trait::async_trait;
use crate::entities::figure::model::Figure;
use crate::utilities::types::IdType;
use interpol::format as iformat;
use sqlx::postgres::PgRow;
use tracing::{trace, instrument};
use crate::entities::figure::traits::FigureRepositoryTrait;
use crate::entities::profile::model::Profile;
use crate::infrastructure::traits::TransactionTrait;
use crate::infrastructure::transaction::PostgresTransaction;

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

#[derive(Debug)]
pub struct FigureAndProfile {
    figure: Figure,
    profile: Profile
}

impl FigureAndProfile {
    pub fn get_figure_and_profile(self) -> (Figure, Profile) {
        (self.figure, self.profile)
    }
}

impl FromRow<'_, PgRow> for FigureAndProfile {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let figure = Figure::from_row(row)?;
        let profile = Profile::from_row(row)?;

        Ok(FigureAndProfile {
            figure,
            profile,
        })
    }
}

#[async_trait]
impl FigureRepositoryTrait<PostgresTransaction> for FigureRepository {
    #[instrument(level = "trace", skip(self, transaction))]
    async fn create(&self, transaction: Option<&mut PostgresTransaction>, mut figure: Figure) -> Result<Figure, ServerError> {
        let query_string = iformat!(r#"
            INSERT INTO figure
            (id, title, description, width, height, url, profile_id)
            VALUES (DEFAULT, $1, $2, $3, $4, $5, $6)
            RETURNING id;
            "#);

        trace!("Query: {}", query_string);

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
            .map_err(|e| ServerError::InternalError(e.into()))
    }

    #[instrument(level = "trace", skip(self, transaction))]
    async fn find_by_id(&self, transaction: Option<&mut PostgresTransaction>, figure_id: IdType) -> Result<(Figure, Profile), ServerError> {
        let query_string = iformat!(r#"
            SELECT
            figure.id AS u_figure_id, figure.title, figure.description,
            figure.url, figure.width, figure.height, figure.profile_id,

            profile.id AS u_profile_id, profile.username, profile.display_name,
            profile.bio, profile.banner, profile.profile_picture, profile.user_id

            FROM figure
            INNER JOIN profile
            ON figure.profile_id = profile.id
            WHERE figure.id = $1
            "#);

        trace!("Query: {}", query_string);

        let query = sqlx::query_as::<_, FigureAndProfile>(&query_string)
            .bind(figure_id);

        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .map(|figure_and_profile| figure_and_profile.get_figure_and_profile())
            .map_err(|e| ServerError::InternalError(e.into()))
    }

    #[instrument(level = "trace", skip(self, transaction))]
    async fn find_starting_from_id_with_profile_id(&self, transaction: Option<&mut PostgresTransaction>, figure_id: Option<IdType>, profile_id: Option<IdType>, limit: i32) -> Result<Vec<(Figure, Profile)>, ServerError> {
        let mut query_string = "
            SELECT figure.id AS u_figure_id, figure.title, figure.description,
            figure.url, figure.width, figure.height, figure.profile_id,

            profile.id AS u_profile_id, profile.username, profile.display_name,
            profile.bio, profile.banner, profile.profile_picture, profile.user_id
            FROM figure
            INNER JOIN profile
            ON figure.profile_id = profile.id
            ".to_string();

        // Select figures with id starting after given figure id
        if let Some(starting_from_id) = figure_id {
            query_string = iformat!(r#"
            {query_string}
            WHERE figure.id < {starting_from_id}
            "#);
        }

        // Filter by given profile id
        if let Some(from_profile) = profile_id {
            let mut filter = iformat!("figure.profile_id = {from_profile}");
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
        ORDER BY figure.id DESC
        LIMIT {limit}
        "#);

        trace!("Query: {}", query_string);

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
            e => ServerError::InternalError(e.into())
        })
    }

    #[instrument(level = "trace", skip(self, transaction))]
    async fn update_figure(&self, transaction: Option<&mut PostgresTransaction>, figure: Figure) -> Result<(), ServerError> {
        let query_string = iformat!(r#"
            UPDATE figure
            SET title = $2, description = $3, url = $4, width = $5, height = $6
            WHERE id = $1
            "#);

        trace!("Query: {}", query_string);

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
            .map_err(|e| ServerError::InternalError(e.into()))
    }

    #[instrument(level = "trace", skip(self, transaction))]
    async fn delete_figure_by_id(&self, transaction: Option<&mut PostgresTransaction>, figure_id: IdType) -> Result<(), ServerError> {
        let query_string = iformat!(r#"
            DELETE FROM figure
            WHERE id = $1
            "#);

        trace!("Query: {}", query_string);

        let query =
            sqlx::query(&query_string)
                .bind(figure_id);

        match transaction {
            Some(transaction) => query.execute(transaction.inner()).await,
            None => query.execute(&self.db).await
        }
            .map(|_result| ())
            .map_err(|e| ServerError::InternalError(e.into()))
    }

    #[instrument(level = "trace", skip(self, transaction))]
    async fn count_by_profile_id(&self, transaction: Option<&mut PostgresTransaction>, profile_id: IdType) -> Result<IdType, ServerError> {
        let query_string = iformat!(r#"
        SELECT count(*) FROM figure
        where profile_id = $1
        "#);

        trace!("Query: {}", query_string);

        let query =
            sqlx::query(&query_string)
                .bind(profile_id);

        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .and_then(|row| row.try_get(0))
            .map_err(|e| ServerError::InternalError(e.into()))
    }

    #[instrument(level = "trace", skip(self, transaction))]
    async fn get_total_figures_count(&self, transaction: Option<&mut PostgresTransaction>) -> Result<IdType, ServerError> {
        let query_string = iformat!(r#"
        SELECT count(*) FROM figure
        "#);

        trace!("Query: {}", query_string);

        let query =
            sqlx::query(&query_string);

        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .and_then(|row| row.try_get(0))
            .map_err(|e| ServerError::InternalError(e.into()))
    }
}