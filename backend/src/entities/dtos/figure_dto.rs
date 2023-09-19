use serde::Serialize;
use serde_json::{json, Value};
use sqlx::{Error, FromRow};
use sqlx::postgres::PgRow;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::domain::models::figure::Figure;
use crate::domain::models::profile::Profile;
use crate::domain::models::types::IdType;

#[derive(Serialize, Debug)]
pub struct FigureDTO {
    pub id: IdType,
    pub title: String,
    pub description: Option<String>,
    pub width: i32,
    pub height: i32,
    pub url: String,
    pub profile: ProfileDTO
}

impl FigureDTO {
    pub fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }

    pub fn to_json(&self) -> Value {
        json!({
            "figure": &self
        })
    }

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

impl FromRow<'_, PgRow> for FigureDTO {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let figure = Figure::from_row(row)?;
        let profile = Profile::from_row(row)?;
        let profile_dto = ProfileDTO::from(profile);

        Ok(FigureDTO {
            id: figure.get_id(),
            title: figure.get_title().to_string(),
            description: figure.get_description().cloned(),
            width: figure.get_width(),
            height: figure.get_height(),
            url: figure.get_url().to_string(),
            profile: profile_dto,
        })
    }
}