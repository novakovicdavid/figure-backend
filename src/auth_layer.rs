use std::sync::{Arc};
use argon2::Algorithm::Argon2id;
use argon2::{Argon2, Params, PasswordHasher};
use argon2::password_hash::SaltString;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use log::error;
use rand_core::OsRng;
use tower_cookies::Cookies;
use unicode_segmentation::UnicodeSegmentation;
use zeroize::Zeroize;
use crate::{ServerState, Session, SessionOption};
use crate::server_errors::ServerError;

pub fn hash_password(password: &str, with_checks: bool) -> Result<String, ServerError> {
    if with_checks {
        let mut password_length = password.graphemes(true).count();
        if password_length < 8 {
            return Err(ServerError::PasswordTooShort)
        }
        if password_length > 60 {
            return Err(ServerError::PasswordTooLong)
        }
        password_length.zeroize();
    }

    let password_salt = SaltString::generate(&mut OsRng);
    let argon2_params = match Params::new(8192, 5, 1, Some(32)) {
        Ok(argon2_params) => argon2_params,
        Err(e) => {
            error!("{}", e);
            return Err(ServerError::InternalError)
        }
    };
    let password_hash = match Argon2::new(Argon2id, argon2::Version::V0x13, argon2_params).hash_password(password.as_ref(), &password_salt) {
        Ok(password_hash) => password_hash,
        Err(e) => {
            error!("{}", e);
            return Err(ServerError::InternalError)
        }
    };
    Ok(password_hash.to_string())
}

pub async fn authenticate<B>(State(server_state): State<Arc<ServerState>>, cookies: Cookies, mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    match cookies.get("session_id") {
        // If the session cookie exists
        Some(cookie) => {
            let session_id = cookie.value();
            // Get the user id associated with the session from the session store
            match server_state.session_store.get_data_of_session(session_id.to_string()).await {
                // If a session is found (and thus hasn't expired
                Ok(session_value) => {
                    // Pass it to the extension so that handlers/extractors can access it
                    req.extensions_mut().insert(SessionOption {
                        session: Some(Session {
                            id: session_id.to_string(),
                            user_id: session_value.user_id,
                            profile_id: session_value.profile_id,
                        })
                    });
                }
                Err(_) => {}
            }
        }
        None => {}
    }

    Ok(next.run(req).await)
}