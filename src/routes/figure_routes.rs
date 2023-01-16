use std::io::Cursor;
use std::sync::Arc;
use anyhow::Context;
use axum::Extension;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use image::GenericImageView;
use serde_json::{json, Value};
use uuid::Uuid;
use crate::entities::types::IdType;
use crate::server_errors::ServerError;
use crate::{ServerState, SessionOption};

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
    let figures = server_state.database.get_figures(starting_from_figure_id, profile_id, &3).await;
    match figures {
        Ok(figures) => {
            json!({
                "figures": figures
            }).to_string().into_response()
        }
        Err(e) => e.into_response()
    }
}

pub async fn upload_figure(session: Extension<SessionOption>, State(server_state): State<Arc<ServerState>>, multipart: Multipart) -> Response {
    let session = match &session.session {
        Some(s) => s,
        None => return StatusCode::UNAUTHORIZED.into_response()
    };

    let result = parse_multipart(multipart).await;
    let (title, description, image, width, height) = match result {
        Ok(tuple) => tuple,
        Err(_) => {
            return ServerError::InvalidMultipart.into_response();
        }
    };

    let uid = Uuid::new_v4();
    let uid = uid.to_string();
    if let Err(e) = server_state.storage.upload_object(uid.as_str(), image).await {
        return ServerError::InternalError(e.to_string()).into_response();
    }

    let url = format!("{}/{}", server_state.storage.get_base_url(), uid);

    match server_state.database.create_figure(title, description, width, height, url, session.profile_id).await {
        Ok(figure_id) => {
            json!({
                "figure_id": figure_id
            }).to_string().into_response()
        }
        Err(_) => ServerError::InternalError("Could not create Figure".to_string()).into_response()
    }
}

async fn parse_multipart(mut multipart: Multipart) -> Result<(String, String, Bytes, i32, i32), anyhow::Error> {
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut image: Option<Bytes> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().context("Multipart parse failed: no field name")?.to_string();
        let data = field.bytes().await?;
        match name.as_str() {
            "title" => title = Some(String::from_utf8(data.to_vec())?),
            "description" => description = Some(String::from_utf8(data.to_vec())?),
            "file" => image = Some(data),
            _ => {}
        };
    };

    if title.is_none() && description.is_none() || image.is_none() {
        return Err(ServerError::MissingFieldInForm)?;
    }
    let title = title.unwrap();
    let description = description.unwrap();
    let image = image.unwrap();

    let format = parse_image_format(&image)?.to_vec();
    if !format.contains(&"jpg") && !format.contains(&"jpeg") && !format.contains(&"png") {
        return Err(ServerError::InvalidImage)?;
    }

    let (width, height) = match get_image_dimensions(&image) {
        Ok(tuple) => tuple,
        Err(e) => {
            return Err(ServerError::InvalidImage)?;
        }
    };

    Ok((title, description, image, width, height))
}

fn get_image_dimensions(image: &Bytes) -> Result<(i32, i32), anyhow::Error> {
    let (width, height) = image::load_from_memory(image)?.dimensions();
    let width = width as i32;
    let height = height as i32;
    Ok((width, height))
}

fn parse_image_format(data: &Bytes) -> Result<&'static [&'static str], anyhow::Error> {
    Ok(image::io::Reader::new(Cursor::new(data))
        .with_guessed_format()?
        .format().context("No format found for image")?
        .extensions_str())
}