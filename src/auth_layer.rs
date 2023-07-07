use std::sync::{Arc};
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;
use crate::{ServerState, Session, SessionOption};
use crate::repositories::traits::SessionRepositoryTrait;

pub async fn authenticate<B>(State(server_state): State<Arc<ServerState>>, cookies: Cookies, mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    if let Some(cookie) = cookies.get("session_id") {
        let session_id = cookie.value();
        // Get the user id associated with the session from the session store
        if let Ok(session_value) = server_state.context.repository_context.session_repository.find_by_id(session_id, Some(86400)).await {
            // Pass it to the extension so that handlers/extractors can access it
            req.extensions_mut().insert(SessionOption {
                session: Some(Session {
                    id: session_value.id,
                    _user_id: session_value._user_id,
                    profile_id: session_value.profile_id,
                })
            });
        }
    }

    Ok(next.run(req).await)
}