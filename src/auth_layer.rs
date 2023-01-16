use std::sync::{Arc};
use argon2::Algorithm::Argon2id;
use argon2::{Argon2, Params, PasswordHasher};
use argon2::password_hash::SaltString;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use lazy_static::lazy_static;
use rand_core::OsRng;
use regex::Regex;
use tower_cookies::Cookies;
use unicode_segmentation::UnicodeSegmentation;
use zeroize::Zeroize;
use crate::{ServerState, Session, SessionOption};
use crate::server_errors::ServerError;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new("^[a-zA-Z0-9_+&*-]+(?:\\.[a-zA-Z0-9_+&*-]+)*@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,}$").unwrap();
    static ref USERNAME_REGEX: Regex = Regex::new("^[a-zA-Z0-9]+-*[a-zA-Z0-9]+?$").unwrap();
}

// Valid email test (OWASP Regex + maximum length of 60 graphemes
pub fn is_email_valid(email: &str) -> bool {
    EMAIL_REGEX.is_match(email) || email.graphemes(true).count() > 60
}

// Valid username test
// (alphanumerical, optionally a dash surrounded by alphanumerical characters, 15 character limit)
pub fn is_username_valid(username: &str) -> bool {
    USERNAME_REGEX.is_match(username) || username.graphemes(true).count() > 15
}

pub fn hash_password(password: &str, with_checks: bool) -> Result<String, ServerError<String>> {
    if with_checks {
        let mut password_length = password.graphemes(true).count();
        if password_length < 8 {
            return Err(ServerError::PasswordTooShort);
        }
        if password_length > 60 {
            return Err(ServerError::PasswordTooLong);
        }
        password_length.zeroize();
    }

    let password_salt = SaltString::generate(&mut OsRng);
    let argon2_params = match Params::new(8192, 5, 1, Some(32)) {
        Ok(argon2_params) => argon2_params,
        Err(e) => {
            return Err(ServerError::InternalError(e.to_string()));
        }
    };
    let password_hash = match Argon2::new(Argon2id, argon2::Version::V0x13, argon2_params).hash_password(password.as_ref(), &password_salt) {
        Ok(password_hash) => password_hash,
        Err(e) => {
            return Err(ServerError::InternalError(e.to_string()));
        }
    };
    Ok(password_hash.to_string())
}

pub async fn authenticate<B>(State(server_state): State<Arc<ServerState>>, cookies: Cookies, mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    if let Some(cookie) = cookies.get("session_id") {
        let session_id = cookie.value();
        // Get the user id associated with the session from the session store
        if let Ok(session_value) = server_state.session_store.get_data_of_session(session_id).await {
            // Pass it to the extension so that handlers/extractors can access it
            req.extensions_mut().insert(SessionOption {
                session: Some(Session {
                    id: session_id.to_string(),
                    _user_id: session_value.user_id,
                    profile_id: session_value.profile_id,
                })
            });
        }
    }

    Ok(next.run(req).await)
}