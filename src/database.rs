// use sqlx::{Pool, Postgres, Transaction};
// use async_trait::async_trait;
// use crate::server_errors::ServerError;
//
// #[async_trait]
// pub trait DatabaseTrait {
//     type TransactionImpl;
//     async fn start_transaction(&self) -> Result<Self::TransactionImpl, ServerError<String>>;
//
// }
//
// #[async_trait]
// pub trait TransactionTrait {
//
// }
//
// pub struct PostgresTransaction {
//     db: Transaction<'static, Postgres>
// }
//
// impl PostgresTransaction {
//     pub fn new(pool: impl DatabaseTrait) -> Self {
//         Self {
//             db:
//         }
//     }
// }
//
// #[async_trait]
// impl TransactionTrait for PostgresTransaction {
//     type TransactionImpl = PostgresTransaction;
//
//     async fn start_transaction(&self) -> Result<Self::TransactionImpl, ServerError<String>> {
//         self.db.begin().await.map_err(|e| ServerError::InternalError(e.to_string()))
//     }
// }