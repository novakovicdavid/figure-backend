use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::entities::types::IdType;
use crate::ServerState;

pub async fn get_figure(State(server_state): State<Arc<ServerState>>, Path(id): Path<IdType>) -> Response {
    let figure = server_state.database.get_figure(&id).await;
    match figure {
        Ok(figure) =>
            Json(figure).into_response(),
        Err(e) => e.into_response()
    }
}