use async_trait::async_trait;
use crate::repositories::transaction::TransactionTrait;
use crate::server_errors::ServerError;

pub struct MockTransaction {
    transaction: ()
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