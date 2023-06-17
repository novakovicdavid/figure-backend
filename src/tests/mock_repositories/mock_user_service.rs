// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use sqlx::{Postgres, Transaction};
// use crate::entities::user::User;
// use crate::repositories::user_repository::UserRepositoryTrait;
// use crate::server_errors::ServerError;
// use async_trait::async_trait;
//
// #[derive(Clone)]
// pub struct MockUserRepository {
//     db: Arc<Mutex<Vec<User>>>
// }
//
// impl MockUserRepository {
//     pub fn new(users: Arc<Mutex<Vec<User>>>) -> impl UserRepositoryTrait {
//         MockUserRepository {
//             db: users
//         }
//     }
// }
//
// #[async_trait]
// impl UserRepositoryTrait for MockUserRepository {
//     async fn start_transaction(&self) -> Result<Transaction<Postgres>, ServerError<String>> {
//
//     }
//
//     async fn create(&self, transaction: Option<&mut Transaction<Postgres>>, email: String, password_hash: String) -> Result<User, ServerError<String>> {
//         let mut db = self.db.lock().unwrap();
//         let user = User {
//             id: 0,
//             email,
//             password: password_hash,
//             role: String::from("user"),
//         };
//         db.push(user.clone());
//         Ok(user)
//     }
//
//     async fn get_user_by_email(&self, transaction: Option<&mut Transaction<Postgres>>, email: String) -> Result<User, ServerError<String>> {
//         let mut db = self.db.lock().unwrap();
//         db.iter().find(|user| user.email == email)
//             .cloned()
//             .ok_or_else(|| ServerError::InternalError(String::from("No user found with email")))
//     }
// }