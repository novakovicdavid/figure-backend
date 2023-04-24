use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use cookie::{Cookie, SameSite};
use serde::Serialize;
use tower_cookies::Cookies;
use crate::{ServerState, Session, SessionOption};
use crate::database::{SignInForm, SignUpForm};
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::types::IdType;
use crate::server_errors::ServerError;

#[derive(Serialize)]
struct SignInResponse {
    profile_id: IdType,
}

impl From<Session> for SignInResponse {
    fn from(session: Session) -> Self {
        SignInResponse {
            profile_id: session.profile_id,
        }
    }
}

pub async fn signin_user(Extension(_session_option): Extension<SessionOption>, State(server_state): State<Arc<ServerState>>, cookies: Cookies, Json(signin): Json<SignInForm>) -> Response {
    return match server_state.database.authenticate_user_by_email(signin.email, signin.password).await {
        Ok((user, profile)) => {
            let session = server_state.session_store.create_session(user.id, profile.id).await.unwrap();
            let mut cookie = Cookie::new("session_id", session.id);
            cookie.set_http_only(true);
            cookie.set_secure(true);
            cookie.set_same_site(SameSite::Strict);
            cookie.set_domain(server_state.domain.to_string());
            cookie.set_path("/");
            cookies.add(cookie);
            ProfileDTO::from(profile).to_json().into_response()
        }
        Err(e) => e.into_response()
    };
}

pub async fn signup_user(State(server_state): State<Arc<ServerState>>, cookies: Cookies, Json(signup): Json<SignUpForm>) -> Response {
    // return match server_state.database.signup_user(signup).await {
    //     Ok((user, profile)) => {
    //         let session = server_state.session_store.create_session(user.id, profile.id).await.unwrap();
    //         let mut cookie = Cookie::new("session_id", session.id);
    //         cookie.set_http_only(true);
    //         cookie.set_secure(true);
    //         cookie.set_same_site(SameSite::Strict);
    //         cookie.set_domain(server_state.domain.to_string());
    //         cookie.set_path("/");
    //         cookies.add(cookie);
    //         ProfileDTO::from(profile).to_json().into_response()
    //     }
    //     Err(e) => e.into_response()
    // }
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
}

pub async fn signout_user(State(server_state): State<Arc<ServerState>>, cookies: Cookies) -> Response {
    if let Some(mut cookie) = cookies.get("session_id") {
        match server_state.session_store.invalidate_session(cookie.value()).await {
            Ok(_) => {
                cookie.set_http_only(true);
                cookie.set_secure(true);
                cookie.set_same_site(SameSite::Strict);
                cookie.set_domain(server_state.domain.to_string());
                cookie.set_path("/");
                cookie.make_removal();
                cookies.add(cookie.into_owned());
                StatusCode::OK.into_response()
            },
            Err(e) => e.into_response()
        }
    }
    else {
        ServerError::NoSessionReceived.into_response()
    }
}

// Return the profile associated with a given session
pub async fn load_session(State(server_state): State<Arc<ServerState>>, cookies: Cookies) -> Response {
    if let Some(cookie) = cookies.get("session_id") {
        match server_state.session_store.get_data_of_session(cookie.value()).await {
            Ok(session_data) => {
                if let Ok(profile) = server_state.database.get_profile_by_id(session_data.profile_id).await {
                    return ProfileDTO::from(profile).to_json().into_response()
                }
                ServerError::ResourceNotFound.into_response()
            }
            Err(_e) => {
                ServerError::NoSessionFound.into_response()
            }
        }
    }
    else {
        ServerError::NoSessionReceived.into_response()
    }
}