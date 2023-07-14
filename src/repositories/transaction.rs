use async_trait::async_trait;
use sqlx::{Pool, Postgres, Transaction};
use crate::repositories::traits::{TransactionCreatorTrait, TransactionTrait};
use crate::server_errors::ServerError;



#[derive(Clone)]
pub struct PostgresTransactionCreator {
    db: Pool<Postgres>
}

impl PostgresTransactionCreator {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self {
            db
        }
    }
}

#[async_trait]
impl TransactionCreatorTrait<PostgresTransaction> for PostgresTransactionCreator {
    async fn create(&self) -> Result<PostgresTransaction, ServerError<String>> {
        self.db.begin().await
            .map(PostgresTransaction::new)
            .map_err(|e| ServerError::InternalError(e.to_string()))
    }
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