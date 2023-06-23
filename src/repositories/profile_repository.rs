use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row, Transaction};
use crate::entities::profile::Profile;
use crate::entities::types::IdType;
use crate::MyTransaction;
use crate::repositories::transaction::{PostgresTransaction, TransactionTrait};
use crate::server_errors::ServerError;

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
pub trait ProfileRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, username: String, user_id: IdType) -> Result<Profile, ServerError<String>>;
    async fn find_by_id(&self, transaction: Option<&mut MyTransaction>, profile_id: IdType) -> Result<Profile, ServerError<String>>;
    async fn find_by_user_id(&self, transaction: Option<&mut MyTransaction>, user_id: IdType) -> Result<Profile, ServerError<String>>;
    async fn update_profile_by_id(&self, transaction: Option<&mut MyTransaction>, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError<String>>;
}

#[async_trait]
impl ProfileRepositoryTrait<PostgresTransaction> for ProfileRepository {
    async fn create(&self, transaction: Option<&mut PostgresTransaction>, username: String, user_id: IdType) -> Result<Profile, ServerError<String>> {
        let query = sqlx::query(r#"
            INSERT INTO profiles (username, user_id)
            VALUES ($1, $2)
            RETURNING id"#)
            .bind(&username)
            .bind(user_id);
        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        };

        match query_result {
            Ok(profile_id) => {
                Ok(
                    Profile {
                        id: profile_id.get(0),
                        username,
                        display_name: None,
                        bio: None,
                        banner: None,
                        profile_picture: None,
                        user_id,
                    }
                )
            }
            Err(e) => {
                match ServerError::parse_db_error(&e) {
                    ServerError::ConstraintError => {
                        Err(ServerError::UsernameAlreadyTaken)
                    }
                    _ => Err(ServerError::InternalError(e.to_string()))
                }
            }
        }
    }

    async fn find_by_id(&self, transaction: Option<&mut MyTransaction>, profile_id: IdType) -> Result<Profile, ServerError<String>> {
        let query =
            sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE id = $1")
                .bind(profile_id);
        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        };
        match query_result {
            Ok(profile) => Ok(profile),
            Err(sqlx::Error::RowNotFound) => Err(ServerError::ResourceNotFound),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn find_by_user_id(&self, transaction: Option<&mut MyTransaction>, user_id: IdType) -> Result<Profile, ServerError<String>> {
        let query =
            sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE user_id = $1")
                .bind(user_id);
        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        };
        match query_result {
            Ok(profile) => Ok(profile),
            Err(sqlx::Error::RowNotFound) => Err(ServerError::ResourceNotFound),
            Err(e) => Err(ServerError::InternalError(e.to_string()))
        }
    }

    async fn update_profile_by_id(&self, transaction: Option<&mut MyTransaction>, profile_id: IdType, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>) -> Result<(), ServerError<String>> {
        let query =
            sqlx::query(r#"
            UPDATE profiles
            SET display_name = $1, bio = $2, banner = $3, profile_picture = $4
            WHERE profiles.id = $5
            "#).bind(display_name)
                .bind(bio)
                .bind(banner)
                .bind(profile_picture)
                .bind(profile_id);

        match transaction {
            Some(transaction) => query.execute(transaction.inner()).await,
            None => query.execute(&self.db).await
        }
            .map(|_result| ())
            .map_err(|e| ServerError::InternalError(e.to_string()))
    }
}