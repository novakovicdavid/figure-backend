// use anyhow::Error;
// use axum::Router;
// use hyper::header::{ACCEPT, CONTENT_TYPE};
// use hyper::{Body, Request, StatusCode};
// use serde_json::json;
// use tower::ServiceExt;
// use super::test_routes_unauthenticated::setup as statesetup;
//
// async fn setup() -> Result<Router, Error> {
//     let state = statesetup().await?;
//
//     // create an account
//     state
//         .clone()
//         .oneshot(Request::builder()
//             .method("POST")
//             .uri("/users/signup")
//             .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
//             .header(ACCEPT, mime::APPLICATION_JSON.as_ref())
//             .body(Body::from(
//                 json!({
//                 "email": "four@four.four",
//                 "username": "four",
//                 "password": "password"
//             }).to_string()
//             ))?)
//         .await?;
//
//     Ok(state)
// }
//
// #[tokio::test]
// async fn test_signin_user() -> Result<(), Error> {
//     let app = setup().await?;
//     let response = app
//         .oneshot(Request::builder()
//             .method("POST")
//             .uri("/users/signin")
//             .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
//             .header(ACCEPT, mime::APPLICATION_JSON.as_ref())
//             .body(Body::from(
//                 json!({
//                     "email": "four@four.four",
//                     "password": "password"
//                 }).to_string()
//             ))?)
//         .await?;
//     assert_eq!(response.status(), StatusCode::OK);
//     Ok(())
// }
// TODO