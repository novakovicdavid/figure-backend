// use argon2::{Argon2, Params, PasswordHasher};
// use argon2::Algorithm::Argon2id;
// use argon2::password_hash::SaltString;
//
// use lazy_static::lazy_static;
// use regex::Regex;
// use unicode_segmentation::UnicodeSegmentation;
// use crate::entities::user::User;
// use crate::server_errors::ServerError;
// use rand_core::OsRng;
// use sqlx::{Postgres, Transaction};
// use crate::repositories::user_repository::UserRepositoryTrait;
// use async_closure::{async_closure_mut as cb, capture_lifetimes::AsyncFnMut};
// use crate::repositories::user_repository::UserRepository;
//
//
// lazy_static! {
//     static ref EMAIL_REGEX: Regex = Regex::new("^[a-zA-Z0-9_+&*-]+(?:\\.[a-zA-Z0-9_+&*-]+)*@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,}$").unwrap();
//     static ref USERNAME_REGEX: Regex = Regex::new("^[a-zA-Z0-9]+-*[a-zA-Z0-9]+?$").unwrap();
// }
//
// trait UserServiceTrait: Send + Sync {
//     async fn signup_user(&self, email: String, password: String, username: String) -> Result<User, ServerError<String>>;
// }
//
// struct UserService<T: UserRepositoryTrait + Send + Sync> {
//     user_repository: T,
// }
//
// impl<T: UserRepositoryTrait + Send + Sync> UserServiceTrait for UserService<T> {
//     async fn signup_user(&self, email: String, password: String, username: String) -> Result<User, ServerError<String>> {
//         if !is_email_valid(&email) {
//             return Err(ServerError::InvalidEmail);
//         }
//         if !is_username_valid(&username) {
//             return Err(ServerError::InvalidUsername);
//         }
//
//         let password_hash_result = hash_password(&password, true);
//         let password_hash = match password_hash_result {
//             Ok(hash) => hash,
//             Err(e) => return Err(e)
//         };
//
//         // async fn signup_user_transaction<T: UserRepositoryTrait>(user_service: UserService<T>, transaction: Transaction<'_, Postgres>, email: String, password_hash: String, username: String) -> Result<User, ServerError<String>>{
//         //     user_service.user_repository.create(Some(&transaction), email, password_hash, username).await
//         // }
//
//         // let queries = |transaction: &Transaction<Postgres>| Box::pin(async move {
//         //     self.user_repository.create(Some(&transaction), email, password_hash, username).await
//         // }).boxed();
//
//
//         // self.user_repository.start_transaction(
//         //     cb!({}; async |transaction: &Transaction<'_, Postgres>| -> Result<User, ServerError<String>> {
//         //         self.user_repository.create(Some(&transaction), email, password_hash, username).await
//         //     })).await
//         self.user_repository.start_transaction(self.user_repository.clone(),
//             cb!({}; async |repository: UserRepository, transaction: &Transaction<'_, Postgres>| -> Result<User, ServerError<String>> {
//                 Err(ServerError::TransactionFailed)
//             })).await
//         // Err(ServerError::TransactionFailed)
//     }
// }
//
// // Valid email test (OWASP Regex + maximum length of 60 graphemes
// fn is_email_valid(email: &str) -> bool {
//     EMAIL_REGEX.is_match(email) || email.graphemes(true).count() > 60
// }
//
// // Valid username test
// // (alphanumerical, optionally a dash surrounded by alphanumerical characters, 15 character limit)
// fn is_username_valid(username: &str) -> bool {
//     USERNAME_REGEX.is_match(username) || username.graphemes(true).count() > 15
// }
//
// fn hash_password(password: &str, with_checks: bool) -> Result<String, ServerError<String>> {
//     if with_checks {
//         let password_length = password.graphemes(true).count();
//         if password_length < 8 {
//             return Err(ServerError::PasswordTooShort);
//         }
//         if password_length > 60 {
//             return Err(ServerError::PasswordTooLong);
//         }
//     }
//
//     let password_salt = SaltString::generate(&mut OsRng);
//     let argon2_params = match Params::new(8192, 5, 1, Some(32)) {
//         Ok(argon2_params) => argon2_params,
//         Err(e) => {
//             return Err(ServerError::InternalError(e.to_string()));
//         }
//     };
//     let password_hash = match Argon2::new(Argon2id, argon2::Version::V0x13, argon2_params).hash_password(password.as_ref(), &password_salt) {
//         Ok(password_hash) => password_hash,
//         Err(e) => {
//             return Err(ServerError::InternalError(e.to_string()));
//         }
//     };
//     Ok(password_hash.to_string())
// }
//
// async fn yo<'a>(transaction: &'a Transaction<'_, Postgres>) -> Result<User, ServerError<String>> {
//     Err(ServerError::TransactionFailed)
// }