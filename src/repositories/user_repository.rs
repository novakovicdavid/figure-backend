// use std::future::Future;
// use std::pin::Pin;
// use async_closure::capture_lifetimes::AsyncFnMut;
// use sqlx::{Pool, Postgres, Row, Transaction};
//
// use crate::entities::user::{User};
// use crate::server_errors::ServerError;
//
// pub struct UserRepository {
//     db: Pool<Postgres>,
// }
//
// pub trait UserRepositoryTrait: Send + Sync + Clone {
//     async fn start_transaction<'env, F, RES>(&self, repository: Self, f: F) -> Result<RES, ServerError<String>>
//         where
//             F: for<'any> AsyncFnMut<
//                 'env, (UserRepository, &'any Transaction<'any, Postgres>), Output=Result<RES, ServerError<String>>,
//             >,
//             RES: Send;
//     async fn create<'a>(&self, transaction: Option<&'a Transaction<Postgres>>, email: String, password_hash: String, username: String) -> Result<User, ServerError<String>>;
//     async fn yo(&self) -> Result<User, ServerError<String>>;
//     // async fn get_user_by_email(&self, email: String) -> Result<User, ServerError<String>>;
// }
//
// impl Clone for UserRepository {
//     fn clone(&self) -> Self {
//         Self {
//             db: self.db.clone()
//         }
//     }
// }
//
// impl UserRepositoryTrait for UserRepository {
//     async fn start_transaction<'env, F, REP, RES>(&self, repository: REP, mut f: F) -> Result<RES, ServerError<String>>
//         where
//             F: for<'any> AsyncFnMut<
//                 'env, (REP, &'any Transaction<'any, Postgres>), Output=Result<RES, ServerError<String>>,
//             >,
//             REP: UserRepositoryTrait,
//             RES: Send {
//         let transaction_result = self.db.begin().await;
//         if let Ok(transaction) = transaction_result {
//             let future_result = f.call_mut((repository, &transaction)).await;
//             if let Ok(result) = future_result {
//                 let commit_result = transaction.commit().await.map_err(|e| ServerError::InternalError(e.to_string()));
//                 if commit_result.is_ok() {
//                     return Ok(result);
//                 }
//                 return Err(ServerError::TransactionFailed);
//             }
//             Err(ServerError::TransactionFailed)
//         } else {
//             Err(ServerError::TransactionFailed)
//         }
//     }
//
//     async fn create<'a>(&self, transaction: Option<&'a Transaction<'_, Postgres>>, email: String, password_hash: String, username: String) -> Result<User, ServerError<String>> {
//         let query = sqlx::query(r#"
//             INSERT INTO users (email, password, role)
//             VALUES ($1, $2, 'user')
//             RETURNING id"#)
//             .bind(email.to_lowercase())
//             .bind(&password_hash);
//         let query_result = query.fetch_one(&self.db).await;
//         // let query_result = match transaction {
//         //     Some(mut transaction) => query.fetch_one(transaction).await,
//         //     None => query.fetch_one(&self.db).await
//         // };
//
//         return match query_result {
//             Ok(user_id) => {
//                 Ok(
//                     User {
//                         email,
//                         password: password_hash,
//                         role: "user".to_string(),
//                         id: user_id.get(0),
//                     }
//                 )
//             }
//             Err(e) => {
//                 return match ServerError::parse_db_error(&e) {
//                     ServerError::ConstraintError => {
//                         Err(ServerError::EmailAlreadyInUse)
//                     }
//                     _ => Err(ServerError::InternalError(e.to_string()))
//                 };
//             }
//         };
//     }
//
//     async fn yo(&self) -> Result<User, ServerError<String>> {
//         Err(ServerError::TransactionFailed)
//     }
//
//
//     // async fn get_user_by_email(&self, email: String) -> Result<User, ServerError<String>> {
//     //     let result =
//     //         sqlx::query_as::<_, User>(&format!("SELECT id, email, password FROM users WHERE email = $1"))
//     //             .bind(email)
//     //             .fetch_one(&self.db).await;
//     //     let result = result.map_err(|e| {
//     //
//     //         return ServerError::ResourceNotFound(e.to_string());
//     //     });
//     //     result
//     // }
// }