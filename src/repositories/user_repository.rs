use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row, Transaction};
use crate::entities::user::{User};
use crate::server_errors::ServerError;
use dyn_clone::DynClone;

#[derive(Clone)]
pub struct UserRepository {
    db: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> impl UserRepositoryTrait {
        UserRepository {
            db: pool
        }
    }
}

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync + DynClone {
    async fn start_transaction(&self) -> Result<Transaction<Postgres>, ServerError<String>>;
    async fn create(&self, transaction: Option<&mut Transaction<Postgres>>, email: String, password_hash: String) -> Result<User, ServerError<String>>;
    async fn get_user_by_email(&self, transaction: Option<&mut Transaction<Postgres>>, email: String) -> Result<User, ServerError<String>>;
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn start_transaction(&self) -> Result<Transaction<Postgres>, ServerError<String>> {
        match self.db.begin().await {
            Ok(transaction) => Ok(transaction),
            Err(e) => Err(ServerError::TransactionFailed)
        }
    }

    async fn create(&self, transaction: Option<&mut Transaction<Postgres>>, email: String, password_hash: String) -> Result<User, ServerError<String>> {
        let query = sqlx::query(r#"
            INSERT INTO users (email, password, role)
            VALUES ($1, $2, 'user')
            RETURNING id"#)
            .bind(email.to_lowercase())
            .bind(&password_hash);
        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction).await,
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

    async fn get_user_by_email(&self, transaction: Option<&mut Transaction<Postgres>>, email: String) -> Result<User, ServerError<String>> {
        let query =
            sqlx::query_as::<_, User>("SELECT id, email, password FROM users WHERE email = $1")
                .bind(email);
        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction).await,
            None => query.fetch_one(&self.db).await
        };
        query_result.map_err(|_e| {
            ServerError::ResourceNotFound
        })
    }
}