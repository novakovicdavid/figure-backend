use std::ops::Deref;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use cookie::{Cookie, SameSite};
use serde::Serialize;
use serde::Deserialize;
use tower_cookies::Cookies;
use crate::context::{ContextTrait, RepositoryContextTrait, ServiceContextTrait};
use crate::ServerState;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::dtos::session_dtos::{Session, SessionOption};
use crate::domain::models::types::IdType;
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

pub async fn sign_in<C: ContextTrait>(Extension(_session_option): Extension<SessionOption>, State(server_state): State<Arc<ServerState<C>>>, cookies: Cookies, Json(signin): Json<SignInForm>) -> Response {
    return match server_state.context.service_context().user_service().sign_in(&signin.email, &signin.password).await {
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

pub async fn sign_up<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, cookies: Cookies, Json(signup): Json<SignUpForm>) -> Response {
    return match server_state.context.service_context().user_service().sign_up(&signup.email, &signup.password, &signup.username).await {
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

pub async fn sign_out<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, cookies: Cookies) -> Response {
    if let Some(mut cookie) = cookies.get("session_id") {
        match server_state.context.repository_context().session_repository().remove_by_id(cookie.value()).await {
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
pub async fn load_session<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, cookies: Cookies) -> Response {
    if let Some(cookie) = cookies.get("session_id") {
        match server_state.context.repository_context().session_repository().find_by_id(cookie.value(), Some(86400)).await {
            Ok(session_data) => {
                server_state.context.service_context().profile_service().find_profile_by_id(session_data.get_profile_id())
                    .await
                    .map(|profile| ProfileDTO::from(profile).to_json())
                    .into_response()
            }
            Err(e) => e.into_response()
        }
    } else {
        ServerError::NoSessionReceived.into_response()
    }
}