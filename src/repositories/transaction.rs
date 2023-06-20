use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use crate::server_errors::ServerError;

#[async_trait]
pub trait TransactionTrait: Send + Sync {
    type Inner;
    async fn commit(self) -> Result<(), ServerError<String>>;
    fn inner(&mut self) -> &mut Self::Inner;
}

pub struct PostgresTransaction {
    transaction: Transaction<'static, Postgres>
}

impl PostgresTransaction {
    pub fn new(transaction: Transaction<'static, Postgres>) -> Self {
        Self {
            transaction
        }
    }
}

#[async_trait]
impl TransactionTrait for PostgresTransaction {
    type Inner = Transaction<'static, Postgres>;
    async fn commit(self) -> Result<(), ServerError<String>> {
        self.transaction.commit().await
            .map_err(|e| ServerError::InternalError(e.to_string()))
    }

    fn inner(&mut self) -> &mut Self::Inner {
        &mut self.transaction
    }
}