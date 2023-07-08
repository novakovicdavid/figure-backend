use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use cookie::{Cookie, SameSite};
use serde::Serialize;
use serde::Deserialize;
use tower_cookies::Cookies;
use crate::ServerState;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::dtos::session_dtos::{Session, SessionOption};
use crate::entities::types::IdType;
use crate::repositories::traits::SessionRepositoryTrait;
use crate::server_errors::ServerError;
use crate::services::traits::{ProfileServiceTrait, UserServiceTrait};

#[derive(Deserialize)]
pub struct SignUpForm {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct SignInForm {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct SignInResponse {
    profile_id: IdType,
}

impl From<Session> for SignInResponse {
    fn from(session: Session) -> Self {
        SignInResponse {
            profile_id: session.get_profile_id(),
        }
    }
}

pub async fn signin_user(Extension(_session_option): Extension<SessionOption>, State(server_state): State<Arc<ServerState>>, cookies: Cookies, Json(signin): Json<SignInForm>) -> Response {
    return match server_state.context.service_context.user_service.authenticate_user(signin.email, signin.password).await {
        Ok((profile, session)) => {
            let mut cookie = Cookie::new("session_id", session.get_id());
            cookie.set_http_only(true);
            cookie.set_secure(true);
            cookie.set_same_site(SameSite::Strict);
            cookie.set_domain(server_state.domain.to_string());
            cookie.set_path("/");
            cookies.add(cookie);
            profile.to_json().into_response()
        }
        Err(e) => e.into_response()
    };
}

pub async fn signup_user(State(server_state): State<Arc<ServerState>>, cookies: Cookies, Json(signup): Json<SignUpForm>) -> Response {
    return match server_state.context.service_context.user_service.signup_user(signup.email, signup.password, signup.username).await {
        Ok((profile, session)) => {
            let mut cookie = Cookie::new("session_id", session.get_id());
            cookie.set_http_only(true);
            cookie.set_secure(true);
            cookie.set_same_site(SameSite::Strict);
            cookie.set_domain(server_state.domain.to_string());
            cookie.set_path("/");
            cookies.add(cookie);
            profile.to_json().into_response()
        }
        Err(e) => e.into_response()
    };
}

pub async fn signout_user(State(server_state): State<Arc<ServerState>>, cookies: Cookies) -> Response {
    if let Some(mut cookie) = cookies.get("session_id") {
        match server_state.context.repository_context.session_repository.remove_by_id(cookie.value()).await {
            Ok(_) => {
                cookie.set_http_only(true);
                cookie.set_secure(true);
                cookie.set_same_site(SameSite::Strict);
                cookie.set_domain(server_state.domain.to_string());
                cookie.set_path("/");
                cookie.make_removal();
                cookies.add(cookie.into_owned());
                StatusCode::OK.into_response()
            }
            Err(e) => e.into_response()
        }
    } else {
        ServerError::NoSessionReceived.into_response()
    }
}

// Return the profile associated with a given session
pub async fn load_session(State(server_state): State<Arc<ServerState>>, cookies: Cookies) -> Response {
    if let Some(cookie) = cookies.get("session_id") {
        match server_state.context.repository_context.session_repository.find_by_id(cookie.value(), Some(86400)).await {
            Ok(session_data) => {
                if let Ok(profile) = server_state.context.service_context.profile_service.find_profile_by_id(session_data.get_profile_id()).await {
                    return ProfileDTO::from(profile).to_json().into_response();
                }
                ServerError::ResourceNotFound.into_response()
            }
            Err(_e) => {
                ServerError::NoSessionFound.into_response()
            }
        }
    } else {
        ServerError::NoSessionReceived.into_response()
    }
}