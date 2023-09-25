use async_trait::async_trait;
use crate::server_errors::ServerError;

#[async_trait]
pub trait TransactionManagerTrait<T: TransactionTrait>: Send + Sync {
    async fn create(&self) -> Result<T, ServerError>;
}

#[async_trait]
pub trait TransactionTrait: Send + Sync {
    type Inner;
    async fn commit(self) -> Result<(), ServerError>;
    fn inner(&mut self) -> &mut Self::Inner;
}