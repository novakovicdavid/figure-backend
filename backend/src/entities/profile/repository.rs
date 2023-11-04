use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use crate::domain::profile::Profile;
use crate::utilities::types::IdType;
use crate::server_errors::ServerError;
use crate::entities::profile::traits::ProfileRepositoryTrait;
use crate::infrastructure::traits::TransactionTrait;
use crate::infrastructure::transaction::PostgresTransaction;

#[derive(Clone)]
pub struct ProfileRepository {
    db: Pool<Postgres>,
}

impl ProfileRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            db: pool
        }
    }
}

#[async_trait]
impl ProfileRepositoryTrait<PostgresTransaction> for ProfileRepository {
    async fn create(&self, transaction: Option<&mut PostgresTransaction>, profile: Profile) -> Result<Profile, ServerError> {
        let query_string = r#"
        INSERT INTO profile (username, user_id)
        VALUES ($1, $2)
        RETURNING id, username, user_id
        "#;

        let query = sqlx::query_as::<_, Profile>(&query_string)
            .bind(profile.get_username())
            .bind(profile.get_user_id());

        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .map_err(|e| {
                match e {
                    sqlx::Error::Database(e) => {
                        if e.constraint() == Some("profile_username_uindex") {
                            return ServerError::UsernameAlreadyTaken
                        }
                        ServerError::InternalError(e.into())
                    }
                    _ => ServerError::InternalError(e.into())
                }
            })
    }

    async fn find_by_id(&self, transaction: Option<&mut PostgresTransaction>, profile_id: IdType) -> Result<Profile, ServerError> {
        let query_string = "SELECT * FROM profile WHERE id = $1";

        let query =
            sqlx::query_as::<_, Profile>(&query_string)
                .bind(profile_id);

        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        };

        match query_result {
            Ok(profile) => Ok(profile),
            Err(sqlx::Error::RowNotFound) => Err(ServerError::ResourceNotFound),
            Err(e) => Err(ServerError::InternalError(e.into()))
        }
    }

    async fn find_by_user_id(&self, transaction: Option<&mut PostgresTransaction>, user_id: IdType) -> Result<Profile, ServerError> {
        let query_string = "SELECT * FROM profile WHERE user_id = $1";

        let query =
            sqlx::query_as::<_, Profile>(&query_string)
                .bind(user_id);

        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        };

        match query_result {
            Ok(profile) => Ok(profile),
            Err(sqlx::Error::RowNotFound) => Err(ServerError::ResourceNotFound),
            Err(e) => Err(ServerError::InternalError(e.into()))
        }
    }

    async fn update_profile_by_id(&self, transaction: Option<&mut PostgresTransaction>, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError> {
        let query_string = r#"
        UPDATE profile
        SET display_name = $1, bio = $2,
        banner = COALESCE($3, banner),
        profile_picture = COALESCE($4, profile_picture)
        WHERE id = $5
        "#;

        let query =
            sqlx::query(&query_string)
                .bind(display_name)
                .bind(bio)
                .bind(banner)
                .bind(profile_picture)
                .bind(profile_id);

        match transaction {
            Some(transaction) => query.execute(transaction.inner()).await,
            None => query.execute(&self.db).await
        }
            .map(|_result| ())
            .map_err(|e| ServerError::InternalError(e.into()))
    }

    async fn get_total_profiles_count(&self, transaction: Option<&mut PostgresTransaction>) -> Result<IdType, ServerError> {
        let query_string = "SELECT count(*) FROM profile";

        let query = sqlx::query(&query_string);

        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .and_then(|row| row.try_get(0))
            .map_err(|e| ServerError::InternalError(e.into()))
    }
}