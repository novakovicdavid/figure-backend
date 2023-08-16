use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use crate::domain::models::profile::Profile;
use crate::domain::models::types::IdType;
use crate::infrastructure::models::profile::ProfileDef;
use crate::server_errors::ServerError;
use interpol::format as iformat;
use crate::repositories::traits::{ProfileRepositoryTrait, TransactionTrait};
use crate::repositories::transaction::PostgresTransaction;

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
        let query_string = iformat!(r#"
            INSERT INTO {ProfileDef::Table} ({ProfileDef::Username.as_str()}, {ProfileDef::UserId.as_str()})
            VALUES ($1, $2)
            RETURNING {ProfileDef::Id.as_str()}, {ProfileDef::Username.as_str()}, {ProfileDef::UserId.as_str()}"#);
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
                        // TODO don't hardcode this
                        if e.constraint() == Some("profile_username_uindex") {
                            return ServerError::UsernameAlreadyTaken
                        }
                        ServerError::InternalError(Arc::new(e.into()))
                    }
                    _ => ServerError::InternalError(Arc::new(e.into()))
                }
            })
    }

    async fn find_by_id(&self, transaction: Option<&mut PostgresTransaction>, profile_id: IdType) -> Result<Profile, ServerError> {
        let query_string = iformat!("SELECT * FROM {ProfileDef::Table} WHERE {ProfileDef::Id.as_str()} = $1");
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
            Err(e) => Err(ServerError::InternalError(Arc::new(e.into())))
        }
    }

    async fn find_by_user_id(&self, transaction: Option<&mut PostgresTransaction>, user_id: IdType) -> Result<Profile, ServerError> {
        let query_string = iformat!("SELECT * FROM {ProfileDef::Table} WHERE {ProfileDef::UserId.as_str()} = $1");
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
            Err(e) => Err(ServerError::InternalError(Arc::new(e.into())))
        }
    }

    async fn update_profile_by_id(&self, transaction: Option<&mut PostgresTransaction>, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError> {
        let query_string = iformat!(r#"
            UPDATE {ProfileDef::Table}
            SET {ProfileDef::DisplayName.as_str()} = $1, {ProfileDef::Bio.as_str()} = $2,
            {ProfileDef::Banner.as_str()} = COALESCE($3, {ProfileDef::Banner.as_str()}),
            {ProfileDef::ProfilePicture.as_str()} = COALESCE($4, {ProfileDef::ProfilePicture.as_str()})
            WHERE {ProfileDef::Id} = $5
            "#);
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
            .map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }

    async fn get_total_profiles_count(&self, transaction: Option<&mut PostgresTransaction>) -> Result<IdType, ServerError> {
        let query_string = iformat!("SELECT count(*) FROM {ProfileDef::Table}");
        let query = sqlx::query(&query_string);
        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .and_then(|row| row.try_get(0))
            .map_err(|e| ServerError::InternalError(Arc::new(e.into())))
    }
}