use std::io::Cursor;
use std::sync::Arc;
use anyhow::Context;
use axum::Extension;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use image::GenericImageView;
use serde_json::json;
use crate::entities::types::IdType;
use crate::server_errors::ServerError;
use crate::{ServerState, SessionOption};
use crate::services::traits::FigureServiceTrait;

pub async fn get_figure(State(server_state): State<Arc<ServerState>>, Path(id): Path<IdType>) -> Response {
    let figure = server_state.context.service_context.figure_service.find_figure_by_id(id).await;
    match figure {
        Ok(figure) => figure.to_json_string().into_response(),
        Err(e) => e.into_response()
    }
}

pub async fn browse_figures(State(server_state): State<Arc<ServerState>>) -> Response {
    get_figures_with_parameters(State(server_state), None, None).await
}

pub async fn browse_figures_starting_from_figure_id(State(server_state): State<Arc<ServerState>>, Path(starting_from_figure_id): Path<IdType>) -> Response {
    get_figures_with_parameters(State(server_state), Some(starting_from_figure_id), None).await
}

pub async fn browse_figures_from_profile(State(server_state): State<Arc<ServerState>>, Path(profile_id): Path<IdType>) -> Response {
    get_figures_with_parameters(State(server_state), None, Some(profile_id)).await
}

pub async fn browse_figures_from_profile_starting_from_figure_id(State(server_state): State<Arc<ServerState>>, Path((profile_id, starting_from_figure_id)): Path<(IdType, IdType)>) -> Response {
    get_figures_with_parameters(State(server_state), Some(starting_from_figure_id), Some(profile_id)).await
}

async fn get_figures_with_parameters(State(server_state): State<Arc<ServerState>>, starting_from_figure_id: Option<IdType>, profile_id: Option<IdType>) -> Response {
    let figures = server_state.context.service_context.figure_service.find_figures_starting_from_id_with_profile_id(starting_from_figure_id, profile_id, 3).await;
    match figures {
        Ok(figures) => {
            json!({
                "figures": figures
            }).to_string().into_response()
        }
        Err(e) => e.into_response()
    }
}

pub async fn landing_page_figures(State(server_state): State<Arc<ServerState>>) -> Response {
    let figures = server_state.context.service_context.figure_service.find_figures_starting_from_id_with_profile_id(None, None, 9).await;
    match figures {
        Ok(figures) => {
            json!({
                "figures": figures
            }).to_string().into_response()
        }
        Err(e) => e.into_response()
    }
}

pub async fn get_total_figures_count(State(server_state): State<Arc<ServerState>>) -> Response {
    match server_state.context.service_context.figure_service.get_total_figures_count().await {
        Ok(id) => id.to_string().into_response(),
        Err(_) => ServerError::InternalError("Failed to get figure count".to_string()).into_response()
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

    match server_state.context.service_context.figure_service.create(title, description, image, width, height, session.profile_id).await {
        Ok(figure) => {
            json!({
                "figure_id": figure.id
            }).to_string().into_response()
        }
        Err(e) => e.into_response()
    }
}

pub async fn get_total_figures_by_profile(State(server_state): State<Arc<ServerState>>, Path(id): Path<IdType>) -> Response {
    match server_state.context.service_context.figure_service.get_total_figures_by_profile(id).await {
        Ok(total) => total.to_string().into_response(),
        Err(_) => ServerError::InternalError(format!("Could not get total figures for {}", id)).into_response()
    }
}

// fn figure_url_from_name(base_url: String, name: String) -> String {
//     format!("{}{}", base_url, name)
// }

async fn parse_multipart(mut multipart: Multipart) -> Result<(String, Option<String>, Bytes, u32, u32), anyhow::Error> {
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

    if title.is_none() || image.is_none() {
        return Err(ServerError::MissingFieldInForm)?;
    }
    let title = title.unwrap();
    let image = image.unwrap();

    let format = parse_image_format(&image)?.to_vec();
    if !format.contains(&"jpg") && !format.contains(&"jpeg") && !format.contains(&"png") {
        return Err(ServerError::InvalidImage)?;
    }

    let (width, height) = match get_image_dimensions(&image) {
        Ok(tuple) => tuple,
        Err(_e) => {
            return Err(ServerError::InvalidImage)?;
        }
    };

    // Convert to JPEG
    let mut buffer = vec![];
    let parsed_image = image::load_from_memory(&image)?;
    parsed_image.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Jpeg(90))?;

    Ok((title, description, Bytes::from(buffer.to_vec()), width, height))
}

fn get_image_dimensions(image: &Bytes) -> Result<(u32, u32), anyhow::Error> {
    let (width, height) = image::load_from_memory(image)?.dimensions();
    let width = width;
    let height = height;
    Ok((width, height))
}

pub fn parse_image_format(data: &Bytes) -> Result<&'static [&'static str], anyhow::Error> {
    Ok(image::io::Reader::new(Cursor::new(data))
        .with_guessed_format()?
        .format().context("No format found for image")?
        .extensions_str())
}