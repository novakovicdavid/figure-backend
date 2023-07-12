use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use crate::entities::user::{User, UserDef};
use crate::server_errors::ServerError;
use interpol::format as iformat;
use crate::entities::types::IdType;
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

    async fn find_one_by_email(&self, transaction: Option<&mut PostgresTransaction>, email: String) -> Result<User, ServerError<String>> {
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

    async fn find_one_by_id(&self, transaction: Option<&mut PostgresTransaction>, id: IdType) -> Result<User, ServerError<String>> {
        let query_string = iformat!("SELECT {UserDef::Id} AS {UserDef::Id.unique()}, {UserDef::Email}, {UserDef::Password}, {UserDef::Role} FROM {UserDef::Table} WHERE {UserDef::Id.as_str()} = $1");
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