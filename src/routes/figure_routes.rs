use std::sync::Arc;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::{json, Value};
use crate::entities::types::IdType;
use crate::server_errors::ServerError;
use crate::ServerState;

pub async fn get_figure(State(server_state): State<Arc<ServerState>>, Path(id): Path<IdType>) -> Response {
    let figure = server_state.database.get_figure(&id).await;
    match figure {
        Ok(figure) => figure.to_json_string().into_response(),
        Err(e) => e.into_response()
    }
}

pub async fn browse_figures(State(server_state): State<Arc<ServerState>>) -> Response {
    browse_figures_with_parameters(State(server_state), None, None).await
}

pub async fn browse_figures_starting_from_figure_id(State(server_state): State<Arc<ServerState>>, Path(starting_from_figure_id): Path<IdType>) -> Response {
    browse_figures_with_parameters(State(server_state), Some(starting_from_figure_id), None).await
}

pub async fn browse_figures_from_profile(State(server_state): State<Arc<ServerState>>, Path(profile_id): Path<IdType>) -> Response {
    browse_figures_with_parameters(State(server_state), None, Some(profile_id)).await
}

pub async fn browse_figures_from_profile_starting_from_figure_id(State(server_state): State<Arc<ServerState>>, Path((profile_id, starting_from_figure_id)): Path<(IdType, IdType)>) -> Response {
    browse_figures_with_parameters(State(server_state), Some(starting_from_figure_id), Some(profile_id)).await
}

async fn browse_figures_with_parameters(State(server_state): State<Arc<ServerState>>, starting_from_figure_id: Option<IdType>, profile_id: Option<IdType>) -> Response {
    let figures = server_state.database.get_figures(starting_from_figure_id, profile_id, &1).await;
    match figures {
        Ok(figures) => {
            json!({
                "figures": figures
            }).to_string().into_response()
        },
        Err(e) => e.into_response()
    }
}

// pub async fn upload_figure(State(server_state): State<Arc<ServerState>>, mut multipart: Multipart) -> Response {
//
// }