use crate::entities::session::session_dtos::Session;
use crate::entities::profile::dtos::ProfileDTO;
use crate::entities::user::model::User;
use crate::infrastructure::traits::TransactionTrait;
use crate::server_errors::ServerError;
use crate::utilities::types::IdType;
use async_trait::async_trait;

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn sign_up(&self, email: &str, password: &str, username: &str) -> Result<(ProfileDTO, Session), ServerError>;
    async fn sign_in(&self, email: &str, password: &str) -> Result<(ProfileDTO, Session), ServerError>;
}

#[async_trait]
pub trait UserRepositoryTrait<T: TransactionTrait>: Send + Sync + Clone {
    async fn create(&self, transaction: Option<&mut T>, user: User) -> Result<User, ServerError>;
    async fn find_one_by_email(&self, transaction: Option<&mut T>, email: &str) -> Result<User, ServerError>;
    async fn find_by_id(&self, transaction: Option<&mut T>, id: IdType) -> Result<User, ServerError>;
}