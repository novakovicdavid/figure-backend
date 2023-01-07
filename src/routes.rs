use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::{ServerState, Session, SessionOption};
use crate::database::{SignInForm, SignUpForm};
use tower_cookies::{Cookie, Cookies};
use cookie::{SameSite};
use crate::entities::types::{Id, IdType};
use crate::server_errors::ServerError;

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

pub async fn get_figure(State(server_state): State<Arc<ServerState>>, Path(id): Path<IdType>) -> Response {
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
            cookie.set_domain("localhost");
            cookie.set_path("/");
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
            cookie.set_domain("localhost");
            cookie.set_path("/");
            cookies.add(cookie);
            Json(profile).into_response()
        }
        Err(e) => e.into_response()
    }
}

pub async fn signout_user(State(server_state): State<Arc<ServerState>>, cookies: Cookies) -> Response {
    if let Some(mut cookie) = cookies.get("session_id") {
        match server_state.session_store.invalidate_session(cookie.value()).await {
            Ok(_) => {
                cookie.set_same_site(SameSite::Strict);
                cookie.set_domain("localhost");
                cookie.set_path("/");
                cookie.make_removal();
                cookies.add(cookie.into_owned());
                StatusCode::OK.into_response()
            },
            Err(e) => e.into_response()
        }
    }
    else {
        ServerError::NoSessionFound.into_response()
    }
}

// Return the profile associated with a given session
pub async fn load_session(State(server_state): State<Arc<ServerState>>, cookies: Cookies) -> Response {
    if let Some(cookie) = cookies.get("session_id") {
        match server_state.session_store.get_data_of_session(cookie.value()).await {
            Ok(session_data) => {
                if let Ok(profile) = server_state.database.get_profile_dto_by_id(*session_data.profile_id).await {
                    return Json(profile).into_response()
                }
                ServerError::ResourceNotFound.into_response()
            }
            Err(e) => {
                ServerError::NoSessionFound.into_response()
            }
        }
    }
    else {
        ServerError::NoSessionFound.into_response()
    }
}