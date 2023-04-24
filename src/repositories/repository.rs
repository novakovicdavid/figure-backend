use std::future::Future;
use crate::server_errors::ServerError;
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

#[async_trait]
pub trait Repository {
    async fn start_transaction<F, Fut, R>(&self, f: F) -> Result<R, ServerError<String>>
        where F: FnOnce(&Transaction<Postgres>) -> Fut + Send,
              Fut: Future<Output=Result<R, ServerError<String>>> + Send,
              R: Send;
}