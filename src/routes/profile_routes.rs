use std::sync::Arc;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use serde_json::json;
use crate::entities::dtos::profile_dto::ProfileWithoutUserIdDTO;
use crate::entities::types::IdType;
use crate::server_errors::ServerError;
use crate::ServerState;

pub async fn get_profile(State(server_state): State<Arc<ServerState>>, Path(profile_id): Path<IdType>) -> Response {
    let profile = server_state.database.get_profile_by_id(profile_id).await;
    match profile {
        Ok(profile) => {
            json!({
                "profile": ProfileWithoutUserIdDTO::from(profile)
            }).to_string().into_response()
        },
        Err(e) => e.into_response()
    }
}

pub async fn get_total_profiles_count(State(server_state): State<Arc<ServerState>>) -> Response {
    match server_state.database.get_total_profiles_count().await {
        Ok(id) => id.to_string().into_response(),
        Err(_) => ServerError::InternalError("Failed to get profile count".to_string()).into_response()
    }
}