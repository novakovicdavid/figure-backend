use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::{ServerState, Session, SessionOption};
use crate::database::{SignInForm, SignUpForm};
use tower_cookies::{Cookie, Cookies};
use cookie::{SameSite};
use crate::entities::types::Id;

#[derive(Serialize)]
struct SignInResponse {
    profile_id: Id,
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
        Ok(figure) =>
            Json(figure).into_response(),
        Err(e) => e.into_response()
    }
}

pub async fn signin_user(Extension(_session_option): Extension<SessionOption>, State(server_state): State<Arc<ServerState>>, cookies: Cookies, Json(signin): Json<SignInForm>) -> Response {
    return match server_state.database.authenticate_user_by_email(signin.email, signin.password).await {
        Ok((user, profile)) => {
            let session = server_state.session_store.create_session(user.id, profile.id).await.unwrap();
            let mut cookie = Cookie::new("session_id", session.id);
            cookie.set_same_site(SameSite::Strict);
            cookies.add(cookie);
            Json(profile).into_response()
        }
        Err(e) => e.into_response()
    };
}

pub async fn signup_user(State(server_state): State<Arc<ServerState>>, cookies: Cookies, Json(signup): Json<SignUpForm>) -> Response {
    return match server_state.database.signup_user(signup).await {
        Ok((user, profile)) => {
            let session = server_state.session_store.create_session(user.id, profile.id).await.unwrap();
            let mut cookie = Cookie::new("session_id", session.id);
            cookie.set_same_site(SameSite::Strict);
            cookies.add(cookie);
            Json(profile).into_response()
        }
        Err(e) => e.into_response()
    }
}