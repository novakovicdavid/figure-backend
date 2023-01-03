use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::{ServerState, Session, SessionOption};
use crate::database::{SignInForm, SignUpForm};
use tower_cookies::{Cookie, Cookies};
use cookie::{SameSite};

#[derive(Serialize)]
struct ErrorResponse<'a> {
    error: &'a str,
}

#[derive(Serialize)]
struct SignInResponse {
    profile_id: i32,
}

impl From<Session> for SignInResponse {
    fn from(session: Session) -> Self {
        SignInResponse {
            profile_id: session.profile_id,
        }
    }
}

pub async fn get_figure(State(server_state): State<Arc<ServerState>>, Path(id): Path<i32>) -> Response {
    let figure = server_state.database.get_figure(&id).await;
    match figure {
        Some(figure) =>
            Json(figure).into_response(),
        None => StatusCode::NOT_FOUND.into_response()
    }
}

// pub async fn signin_user(Extension(_session_option): Extension<SessionOption>, State(server_state): State<Arc<ServerState>>, cookies: Cookies, Json(mut signin): Json<SignInForm>) -> Response {
//     return match server_state.database.authenticate_user_by_email(signin.email, signin.password).await {
//         Ok(user) => {
//             let session = server_state.session_store.create_session(user.0.id, user.1.id).await.unwrap();
//             let mut cookie = Cookie::new("session_id", session.id);
//             cookie.set_same_site(SameSite::Strict);
//             cookies.add(cookie);
//             Json(user.1).into_response()
//         }
//         Err(e) => {
//             (
//                 StatusCode::BAD_REQUEST,
//                 Json(ErrorResponse {
//                     error: &e.to_string()
//                 })
//             ).into_response()
//         }
//     }
// }
//
// pub async fn signup_user(State(server_state): State<Arc<ServerState>>, cookies: Cookies, Json(signup): Json<SignUpForm>) -> Response {
//     let result = server_state.database.signup_user(signup).await;
//     match result {
//         Ok((user, profile)) => {
//             let session = server_state.session_store.create_session(user.id, profile.id).await.unwrap();
//             let mut cookie = Cookie::new("session_id", session.id);
//             cookie.set_same_site(SameSite::Strict);
//             cookies.add(cookie);
//             Json(profile).into_response()
//         }
//         Err(error) => {
//             (
//                 StatusCode::BAD_REQUEST,
//                 Json(ErrorResponse {
//                     error: &error.to_string()
//                 })
//             ).into_response()
//         }
//     }
// }