use serde::Serialize;
use serde_json::{json, Value};
use sqlx::{Error, FromRow};
use sqlx::postgres::PgRow;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::figure::Figure;
use crate::entities::profile::Profile;
use crate::entities::types::IdType;

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

    pub fn from(figure: Figure, profile_dto: ProfileDTO) -> Self {
        Self {
            id: figure.id,
            title: figure.title,
            description: figure.description,
            width: figure.width,
            height: figure.height,
            url: figure.url,
            profile: profile_dto,
        }
    }
}

impl FromRow<'_, PgRow> for FigureDTO {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let figure = Figure::from_row(row)?;
        let profile = Profile::from_row(row)?;
        let profile_dto = ProfileDTO::from(profile);

        Ok(FigureDTO {
            id: figure.id,
            title: figure.title,
            description: figure.description,
            width: figure.width,
            height: figure.height,
            url: figure.url,
            profile: profile_dto,
        })
    }
}