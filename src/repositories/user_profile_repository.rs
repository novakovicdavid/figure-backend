use std::future::Future;
use sqlx::{Pool, Postgres, Row, Transaction};

use crate::entities::user::{User};
use crate::server_errors::ServerError;
use async_trait::async_trait;
use crate::repositories::repository::Repository;

pub struct UserRepository {
    db: Pool<Postgres>,
}

#[async_trait]
pub trait UserRepositoryTrait: Sync + Send + Repository {
    async fn create(&self, transaction: Option<&Transaction<Postgres>>, email: String, password_hash: String, username: String) -> Result<User, ServerError<String>>;
    // async fn get_user_by_email(&self, email: String) -> Result<User, ServerError<String>>;
}

#[async_trait]
impl Repository for UserRepository {
    async fn start_transaction<F, Fut, R>(&self, f: F) -> Result<R, ServerError<String>>
        where F: FnOnce(&Transaction<Postgres>) -> Fut + Send,
              Fut: Future<Output=Result<R, ServerError<String>>> + Send,
              R: Send {
        let transaction_result = self.db.begin().await;
        if let Ok(transaction) = transaction_result {
            let result = f(&transaction).await;
            if let Ok(result) = result {
                let commit_result = transaction.commit().await.map_err(|e| ServerError::InternalError(e.to_string()));
                if commit_result.is_ok() {
                    return Ok(result);
                }
                return Err(ServerError::TransactionFailed);
            }
            return Err(ServerError::TransactionFailed);
        } else {
            Err(ServerError::TransactionFailed)
        }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, transaction: Option<&Transaction<Postgres>>, email: String, password_hash: String, username: String) -> Result<User, ServerError<String>> {
        let query = sqlx::query(r#"
            INSERT INTO users (email, password, role)
            VALUES ($1, $2, 'user')
            RETURNING id"#)
            .bind(email.to_lowercase())
            .bind(&password_hash);
        let query_result = query.fetch_one(&self.db).await;
        // let query_result = match transaction {
        //     Some(mut transaction) => query.fetch_one(transaction).await,
        //     None => query.fetch_one(&self.db).await
        // };

        return match query_result {
            Ok(user_id) => {
                Ok(
                    User {
                        email,
                        password: password_hash,
                        role: "user".to_string(),
                        id: user_id.get(0),
                    }
                )
            }
            Err(e) => {
                return match ServerError::parse_db_error(&e) {
                    ServerError::ConstraintError => {
                        Err(ServerError::EmailAlreadyInUse)
                    }
                    _ => Err(ServerError::InternalError(e.to_string()))
                };
            }
        };
    }

    // async fn get_user_by_email(&self, email: String) -> Result<User, ServerError<String>> {
    //     let result =
    //         sqlx::query_as::<_, User>(&format!("SELECT id, email, password FROM users WHERE email = $1"))
    //             .bind(email)
    //             .fetch_one(&self.db).await;
    //     let result = result.map_err(|e| {
    //
    //         return ServerError::ResourceNotFound(e.to_string());
    //     });
    //     result
    // }
}