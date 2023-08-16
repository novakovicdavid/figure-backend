use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use crate::domain::models::user::User;
use crate::infrastructure::models::user::UserDef;
use crate::server_errors::ServerError;
use interpol::format as iformat;
use crate::domain::models::types::IdType;
use crate::repositories::traits::{TransactionTrait, UserRepositoryTrait};
use crate::repositories::transaction::PostgresTransaction;

#[derive(Clone)]
pub struct UserRepository {
    db: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        UserRepository {
            db: pool
        }
    }
}

#[async_trait]
impl UserRepositoryTrait<PostgresTransaction> for UserRepository {
    async fn create(&self, transaction: Option<&mut PostgresTransaction>, user: User) -> Result<User, ServerError> {
        let query_string = iformat!(r#"
            INSERT INTO {UserDef::Table} ({UserDef::Email.as_str()}, {UserDef::Password.as_str()}, {UserDef::Role.as_str()})
            VALUES ($1, $2, 'user')
            RETURNING {UserDef::Id.as_str()}, {UserDef::Email.as_str()}, {UserDef::Password.as_str()}, {UserDef::Role.as_str()}"#);
        let query = sqlx::query_as::<_, User>(&query_string)
            .bind(user.get_email())
            .bind(user.get_password());

        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .map_err(|e| {
                match e {
                    sqlx::Error::Database(e) => {
                        // TODO don't hardcode this
                        if e.constraint() == Some("user_email_uindex") {
                            return ServerError::EmailAlreadyInUse
                        }
                        ServerError::InternalError(Arc::new(e.into()))
                    }
                    _ => ServerError::InternalError(Arc::new(e.into()))
                }
            })
    }

    async fn find_one_by_email(&self, transaction: Option<&mut PostgresTransaction>, email: String) -> Result<User, ServerError> {
        let query_string = iformat!(r#"
        SELECT {UserDef::Id} AS {UserDef::Id.unique()}, {UserDef::Email}, {UserDef::Password}, {UserDef::Role}
        FROM {UserDef::Table}
        WHERE {UserDef::Email.as_str()} = $1
        "#);
        let query =
            sqlx::query_as::<_, User>(&query_string)
                .bind(email);
        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        };
        query_result.map_err(|_e| {
            ServerError::ResourceNotFound
        })
    }

    async fn find_by_id(&self, transaction: Option<&mut PostgresTransaction>, id: IdType) -> Result<User, ServerError> {
        let query_string = iformat!(r#"
        SELECT {UserDef::Id} AS {UserDef::Id.unique()}, {UserDef::Email}, {UserDef::Password}, {UserDef::Role}
        FROM {UserDef::Table}
        WHERE {UserDef::Id.as_str()} = $1
        "#);
        let query =
            sqlx::query_as::<_, User>(&query_string)
                .bind(id);
        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        };
        query_result.map_err(|_e| {
            ServerError::ResourceNotFound
        })
    }
}