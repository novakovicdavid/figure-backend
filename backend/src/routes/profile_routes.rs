use std::io::Cursor;
use std::sync::Arc;
use anyhow::Context;
use axum::Extension;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use bytes::Bytes;
use crate::context::{ContextTrait, ServiceContextTrait};
use crate::entities::dtos::profile_dto::ProfileWithoutUserIdDTO;
use crate::entities::dtos::session_dtos::SessionOption;
use crate::domain::models::types::IdType;
use crate::server_errors::ServerError;
use crate::ServerState;
use crate::routes::figure_routes::parse_image_format;
use crate::services::traits::ProfileServiceTrait;
use crate::utilities::to_json_string::to_json_string_with_name;

pub async fn get_profile<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, Path(profile_id): Path<IdType>) -> impl IntoResponse {
    server_state.context.service_context().profile_service()
        .find_profile_by_id(profile_id)
        .await
        .and_then(|profile| Ok(ProfileWithoutUserIdDTO::from(profile)))
        .map(to_json_string_with_name)
}

pub async fn get_total_profiles_count<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>) -> impl IntoResponse {
    server_state.context.service_context().profile_service()
        .get_total_profiles_count()
        .await
        .map(|count| count.to_string())
}

pub async fn update_profile<C: ContextTrait>(State(server_state): State<Arc<ServerState<C>>>, session: Extension<SessionOption>, multipart: Multipart) -> impl IntoResponse {
    // Check if logged in
    let session = match &session.session_opt {
        Some(s) => s,
        None => return StatusCode::UNAUTHORIZED.into_response()
    };

    // Parse multipart
    let result = parse_update_profile_multipart(multipart).await;
    let (display_name, bio, banner, profile_picture) = match result {
        Ok(tuple) => tuple,
        Err(_) => return ServerError::InvalidMultipart.into_response()
    };

    // Update profile
    server_state.context.service_context().profile_service()
        .update_profile_by_id(session.get_profile_id(), display_name, bio, banner, profile_picture)
        .await
        .into_response()
}

async fn parse_update_profile_multipart(mut multipart: Multipart) -> Result<(Option<String>, Option<String>, Option<Bytes>, Option<Bytes>), anyhow::Error> {
    let mut display_name: Option<String> = None;
    let mut bio: Option<String> = None;
    let mut banner_option: Option<Bytes> = None;
    let mut profile_picture_option: Option<Bytes> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().context("Multipart parse failed: no field name")?.to_string();
        let data = field.bytes().await?;

        match name.as_str() {
            "display_name" => display_name = Some(String::from_utf8(data.to_vec())?),
            "bio" => bio = Some(String::from_utf8(data.to_vec())?),
            "banner" => banner_option = Some(data),
            "profile_picture" => profile_picture_option = Some(data),
            _ => {}
        };
    };

    if let Some(banner) = banner_option {
        if std::str::from_utf8(&banner).is_err() {
            let format = parse_image_format(&banner)?.to_vec();
            if !format.contains(&"jpg") && !format.contains(&"jpeg") && !format.contains(&"png") {
                return Err(ServerError::InvalidImage)?;
            }
            // Convert to JPEG
            let mut buffer = vec![];
            let parsed_image = image::load_from_memory(&banner)?;
            parsed_image.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Jpeg(90))?;
            banner_option = Some(Bytes::from(buffer));
        }
        else {
            banner_option = None;
        }
    }
    if let Some(profile_picture) = profile_picture_option {
        if std::str::from_utf8(&profile_picture).is_err() {
            let format = parse_image_format(&profile_picture)?.to_vec();
            if !format.contains(&"jpg") && !format.contains(&"jpeg") && !format.contains(&"png") {
                return Err(ServerError::InvalidImage)?;
            }
            // Convert to JPEG
            let mut buffer = vec![];
            let parsed_image = image::load_from_memory(&profile_picture)?;
            parsed_image.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Jpeg(90))?;
            profile_picture_option = Some(Bytes::from(buffer));
        }
        else {
            profile_picture_option = None;
        }
    }

    Ok((display_name, bio, banner_option, profile_picture_option))
}