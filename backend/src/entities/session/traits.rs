use crate::entities::session::session_dtos::Session;
use crate::server_errors::ServerError;
use async_trait::async_trait;

#[async_trait]
pub trait SessionRepositoryTrait: Send + Sync + Clone {
    async fn create(&self, session: Session) -> Result<Session, ServerError>;
    async fn find_by_id(&self, session_id: &str, time_until_expiration: Option<usize>) -> Result<Session, ServerError>;
    async fn remove_by_id(&self, session_id: &str) -> Result<(), ServerError>;
}