use derive_name::with_name;
use serde::Serialize;
use crate::entities::profile::dtos::ProfileDTO;
use crate::domain::figure::Figure;
use crate::domain::profile::Profile;
use crate::utilities::types::IdType;

#[derive(Serialize, Debug)]
#[with_name(figure)]
pub struct FigureDTO {
    pub id: IdType,
    pub title: String,
    pub description: Option<String>,
    pub width: i32,
    pub height: i32,
    pub url: String,
}

#[derive(Serialize, Debug)]
#[with_name(figure)]
pub struct FigureWithProfileDTO {
    pub id: IdType,
    pub title: String,
    pub description: Option<String>,
    pub width: i32,
    pub height: i32,
    pub url: String,
    pub profile: ProfileDTO,
}

impl FigureWithProfileDTO {
    pub fn from(figure: Figure, profile: Profile) -> Self {
        Self {
            id: figure.get_id(),
            title: figure.get_title().to_string(),
            description: figure.get_description().cloned(),
            width: figure.get_width(),
            height: figure.get_height(),
            url: figure.get_url().to_string(),
            profile: ProfileDTO::from(profile),
        }
    }
}

#[derive(Serialize, Debug)]
#[with_name(figures)]
pub struct FiguresDTO {
    figures: Vec<FigureDTO>,
}

#[derive(Serialize, Debug)]
#[with_name(figures)]
pub struct FiguresWithProfileDTO {
    figures: Vec<FigureWithProfileDTO>,
}

impl FiguresWithProfileDTO {
    pub fn from(figures: Vec<FigureWithProfileDTO>) -> Self {
        Self {
            figures,
        }
    }
}