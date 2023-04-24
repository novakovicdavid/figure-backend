use std::sync::{Arc};
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;
use crate::{ServerState, Session, SessionOption};

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