use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use crate::entities::user::{User, UserDef};
use crate::repositories::transaction::{PostgresTransaction, TransactionTrait};
use crate::server_errors::ServerError;
use interpol::format as iformat;

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
pub trait UserRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, email: String, password_hash: String) -> Result<User, ServerError<String>>;
    async fn get_user_by_email(&self, transaction: Option<&mut T>, email: String) -> Result<User, ServerError<String>>;
}

#[async_trait]
impl UserRepositoryTrait<PostgresTransaction> for UserRepository {
    async fn create(&self, transaction: Option<&mut PostgresTransaction>, email: String, password_hash: String) -> Result<User, ServerError<String>> {
        let query_string = iformat!(r#"
            INSERT INTO {UserDef::Table} ({UserDef::Email.as_str()}, {UserDef::Password.as_str()}, {UserDef::Role.as_str()})
            VALUES ($1, $2, 'user')
            RETURNING {UserDef::Id.as_str()}"#);
        let query = sqlx::query(&query_string)
            .bind(email.to_lowercase())
            .bind(&password_hash);
        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        };

        match query_result {
            Ok(user_id) =>
                Ok(User {
                    email,
                    password: password_hash,
                    role: "user".to_string(),
                    id: user_id.get(0),
                }),
            Err(e) => match ServerError::parse_db_error(&e) {
                ServerError::ConstraintError => {
                    Err(ServerError::EmailAlreadyInUse)
                }
                _ => Err(ServerError::InternalError(e.to_string()))
            }
        }
    }

    async fn get_user_by_email(&self, transaction: Option<&mut PostgresTransaction>, email: String) -> Result<User, ServerError<String>> {
        let query_string = iformat!("SELECT {UserDef::Id} AS {UserDef::Id.unique()}, {UserDef::Email}, {UserDef::Password}, {UserDef::Role} FROM {UserDef::Table} WHERE {UserDef::Email.as_str()} = $1");
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
}