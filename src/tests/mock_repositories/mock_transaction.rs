use async_trait::async_trait;
use crate::repositories::transaction::{TransactionCreator, TransactionTrait};
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
impl TransactionCreator<MockTransaction> for MockTransactionCreator {
    async fn create(&self) -> Result<MockTransaction, ServerError<String>> {
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
    async fn commit(self) -> Result<(), ServerError<String>> {
        Ok(())
    }

    fn inner(&mut self) -> &mut Self::Inner {
        &mut self.transaction
    }
}