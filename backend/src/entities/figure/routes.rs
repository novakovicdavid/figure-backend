use std::io::Cursor;
use std::sync::Arc;
use anyhow::Context;
use axum::Extension;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use image::GenericImageView;
use crate::context::{ContextTrait, ServiceContextTrait};
use crate::entities::session::session_dtos::SessionOption;
use crate::utilities::types::IdType;
use crate::entities::figure::dtos::FiguresWithProfileDTO;
use crate::entities::figure::traits::FigureServiceTrait;
use crate::server_errors::ServerError;
use crate::ServerState;
use crate::utilities::to_json_string::to_json_string;

pub async fn get_figure<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, Path(id): Path<IdType>) -> impl IntoResponse {
    server_state.context.service_context().figure_service()
        .find_figure_by_id(id)
        .await
        .and_then(to_json_string)
}

pub async fn browse_figures<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>) -> impl IntoResponse {
    get_figures_with_parameters(State(server_state), None, None)
        .await
}

pub async fn browse_figures_starting_from_figure_id<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, Path(starting_from_figure_id): Path<IdType>) -> impl IntoResponse {
    get_figures_with_parameters(State(server_state), Some(starting_from_figure_id), None)
        .await
}

pub async fn browse_figures_from_profile<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, Path(profile_id): Path<IdType>) -> impl IntoResponse {
    get_figures_with_parameters(State(server_state), None, Some(profile_id))
        .await
}

pub async fn browse_figures_from_profile_starting_from_figure_id<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, Path((profile_id, starting_from_figure_id)): Path<(IdType, IdType)>) -> impl IntoResponse {
    get_figures_with_parameters(State(server_state), Some(starting_from_figure_id), Some(profile_id))
        .await
}

async fn get_figures_with_parameters<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, starting_from_figure_id: Option<IdType>, profile_id: Option<IdType>) -> impl IntoResponse {
    server_state.context.service_context().figure_service()
        .find_figures_starting_from_id_with_profile_id(starting_from_figure_id, profile_id, 3)
        .await
        .map(FiguresWithProfileDTO::from)
        .and_then(to_json_string)
}

pub async fn landing_page_figures<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>) -> impl IntoResponse {
    server_state.context.service_context().figure_service()
        .find_figures_starting_from_id_with_profile_id(None, None, 9)
        .await
        .map(FiguresWithProfileDTO::from)
        .map(to_json_string)

}

pub async fn get_total_figures_count<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>) -> impl IntoResponse {
    server_state.context.service_context().figure_service().get_total_figures_count()
        .await
        .map(|id| id.to_string())
}

pub async fn upload_figure<C: ContextTrait>(session: Extension<SessionOption>, State(server_state): State<Arc<ServerState<C>>>, multipart: Multipart) -> Response {
    let session = match &session.session_opt {
        Some(s) => s,
        None => return StatusCode::UNAUTHORIZED.into_response()
    };

    let result = parse_multipart(multipart).await;
    let (title, description, image, width, height) = match result {
        Ok(tuple) => tuple,
        Err(_e) => {
            return ServerError::InvalidMultipart.into_response();
        }
    };

    server_state.context.service_context().figure_service()
        .create(title, description, image, width, height, session.get_profile_id())
        .await
        .map(|figure| figure.get_id().to_string())
        .into_response()
}

pub async fn get_total_figures_by_profile<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, Path(id): Path<IdType>) -> impl IntoResponse {
    server_state.context.service_context().figure_service()
        .get_total_figures_by_profile(id)
        .await
        .map(|id| id.to_string())
}

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

    let (title, image) = match (title, image) {
        (Some(title), Some(image)) => (title, image),
        _ => {
            return Err(ServerError::MissingFieldInForm)?;
        }
    };

    let format = parse_image_format(&image)?.to_vec();

    // If format isn't one of these formats in the array
    if !["jpg", "jpeg", "png"].iter().any(|&f| format.contains(&f)) {
        return Err(ServerError::InvalidImage)?;
    }

    let (width, height) = match get_image_dimensions(&image) {
        Ok(tuple) => tuple,
        Err(_e) => {
            return Err(ServerError::InvalidImage)?;
        }
    };

    // Convert to JPEG
    let parsed_image = image::load_from_memory(&image)?;
    let mut buffer = vec![];
    parsed_image.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Jpeg(90))?;

    Ok((title, description, Bytes::from(buffer.to_vec()), width, height))
}

fn get_image_dimensions(image: &Bytes) -> Result<(u32, u32), anyhow::Error> {
    Ok(image::load_from_memory(image)?.dimensions())
}

pub fn parse_image_format(data: &Bytes) -> Result<&'static [&'static str], anyhow::Error> {
    Ok(image::io::Reader::new(Cursor::new(data))
        .with_guessed_format()?
        .format().context("No format found for image")?
        .extensions_str())
}