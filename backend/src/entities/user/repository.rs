use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use crate::entities::user::user::User;
use crate::entities::user::infra::UserDef;
use crate::server_errors::ServerError;
use interpol::format as iformat;
use crate::entities::user::traits::UserRepositoryTrait;
use crate::infrastructure::traits::TransactionTrait;
use crate::utilities::types::IdType;
use crate::infrastructure::transaction::PostgresTransaction;

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
            INSERT INTO {UserDef::TABLE} ({UserDef::EMAIL} {UserDef::PASSWORD}, {UserDef::ROLE})
            VALUES ($1, $2, 'user')
            RETURNING {UserDef::ID}, {UserDef::EMAIL}, {UserDef::PASSWORD}, {UserDef::ROLE}"#);
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
                        ServerError::InternalError(e.into())
                    }
                    _ => ServerError::InternalError(e.into())
                }
            })
    }

    async fn find_one_by_email(&self, transaction: Option<&mut PostgresTransaction>, email: &str) -> Result<User, ServerError> {
        let query_string = iformat!(r#"
        SELECT {UserDef::ID} AS {UserDef::ID_UNIQUE}, {UserDef::EMAIL}, {UserDef::PASSWORD}, {UserDef::ROLE}
        FROM {UserDef::TABLE}
        WHERE {UserDef::EMAIL} = $1
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
        SELECT {UserDef::ID} AS {UserDef::ID_UNIQUE}, {UserDef::EMAIL}, {UserDef::PASSWORD}, {UserDef::ROLE}
        FROM {UserDef::TABLE}
        WHERE {UserDef::ID} = $1
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