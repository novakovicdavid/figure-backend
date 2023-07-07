use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use crate::entities::types::IdType;
use crate::entities::user::User;
use crate::repositories::traits::UserRepositoryTrait;
use crate::server_errors::ServerError;
use crate::tests::mock_repositories::mock_transaction::MockTransaction;

#[derive(Clone)]
pub struct MockUserRepository {
    db: Arc<Mutex<Vec<User>>>
}

impl MockUserRepository {
    pub fn new(users: Arc<Mutex<Vec<User>>>) -> Self {
        MockUserRepository {
            db: users
        }
    }
}

#[async_trait]
impl UserRepositoryTrait<MockTransaction> for MockUserRepository {
    async fn create(&self, _transaction: Option<&mut MockTransaction>, email: String, password_hash: String) -> Result<User, ServerError<String>> {
        let mut db = self.db.lock().unwrap();
        let user = User {
            id: db.len() as IdType,
            email,
            password: password_hash,
            role: String::from("user"),
        };
        db.push(user.clone());
        Ok(user)
    }

    async fn get_user_by_email(&self, _transaction: Option<&mut MockTransaction>, email: String) -> Result<User, ServerError<String>> {
        let db = self.db.lock().unwrap();
        db.iter().find(|user| user.email == email)
            .cloned()
            .ok_or_else(|| ServerError::InternalError(String::from("No user found with email")))
    }
}