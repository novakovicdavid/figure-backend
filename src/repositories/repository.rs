use std::future::Future;
use std::pin::Pin;
use async_closure::capture_lifetimes::AsyncFnMut;
use crate::server_errors::ServerError;
use sqlx::{Postgres, Transaction};


// pub trait Repository {
//     async fn start_transaction<'env, F, REP, RES>(&self, repository: REP, f: F) -> Result<RES, ServerError<String>>
//         where F: for<'any> AsyncFnMut<
//             'env,
//             &'any Transaction<'any, Postgres>,
//             Output=Result<RES, ServerError<String>>,
//         >,
//               REP: Repository,
//               RES: Send;
// }