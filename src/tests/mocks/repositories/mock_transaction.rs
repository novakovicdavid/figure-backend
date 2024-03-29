use async_trait::async_trait;
use crate::repositories::traits::{TransactionCreatorTrait, TransactionTrait};
use crate::server_errors::ServerError;

pub struct MockTransactionCreator {
    _db: (),
}

impl MockTransactionCreator {
    pub fn new() -> Self {
        Self {
            _db: (),
        }
    }
}

#[async_trait]
impl TransactionCreatorTrait<MockTransaction> for MockTransactionCreator {
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