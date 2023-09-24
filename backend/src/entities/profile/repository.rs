use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use crate::entities::profile::model::Profile;
use crate::utilities::types::IdType;
use crate::entities::profile::infra::ProfileDef;
use crate::server_errors::ServerError;
use interpol::format as iformat;
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
        let query_string = iformat!(r#"
            INSERT INTO {ProfileDef::TABLE} ({ProfileDef::USERNAME}, {ProfileDef::USER_ID})
            VALUES ($1, $2)
            RETURNING {ProfileDef::ID}, {ProfileDef::USERNAME}, {ProfileDef::USER_ID}"#);
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
                        ServerError::InternalError(e.into())
                    }
                    _ => ServerError::InternalError(e.into())
                }
            })
    }

    async fn find_by_id(&self, transaction: Option<&mut PostgresTransaction>, profile_id: IdType) -> Result<Profile, ServerError> {
        let query_string = iformat!("SELECT * FROM {ProfileDef::TABLE} WHERE {ProfileDef::ID} = $1");
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
        let query_string = iformat!("SELECT * FROM {ProfileDef::TABLE} WHERE {ProfileDef::USER_ID} = $1");
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
        let query_string = iformat!(r#"
            UPDATE {ProfileDef::TABLE}
            SET {ProfileDef::DISPLAY_NAME} = $1, {ProfileDef::BIO} = $2,
            {ProfileDef::BANNER} = COALESCE($3, {ProfileDef::BANNER}),
            {ProfileDef::PROFILE_PICTURE} = COALESCE($4, {ProfileDef::PROFILE_PICTURE})
            WHERE {ProfileDef::ID} = $5
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
            .map_err(|e| ServerError::InternalError(e.into()))
    }

    async fn get_total_profiles_count(&self, transaction: Option<&mut PostgresTransaction>) -> Result<IdType, ServerError> {
        let query_string = iformat!("SELECT count(*) FROM {ProfileDef::TABLE}");
        let query = sqlx::query(&query_string);
        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .and_then(|row| row.try_get(0))
            .map_err(|e| ServerError::InternalError(e.into()))
    }
}