use async_trait::async_trait;
use crate::infrastructure::traits::{TransactionManagerTrait, TransactionTrait};
use crate::server_errors::ServerError;

pub struct MockTransactionManager {
    _db: (),
}

impl MockTransactionManager {
    pub fn new() -> Self {
        Self {
            _db: (),
        }
    }
}

#[async_trait]
impl TransactionManagerTrait<MockTransaction> for MockTransactionManager {
    async fn create(&self) -> Result<MockTransaction, ServerError> {
        Ok(MockTransaction::new())
    }
}

pub struct MockTransaction {
    transaction: (),
}

impl MockTransaction {
    pub fn new() -> Self {
        Self {
            transaction: (),
        }
    }
}

#[async_trait]
impl TransactionTrait for MockTransaction {
    type Inner = ();
    async fn commit(self) -> Result<(), ServerError> {
        Ok(())
    }

    fn inner(&mut self) -> &mut Self::Inner {
        &mut self.transaction
    }
}