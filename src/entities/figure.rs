use serde::{Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::entities::types::IdType;

#[derive(Serialize, Debug)]
pub struct Figure {
    pub id: IdType,
    pub title: String,
    pub description: Option<String>,
    pub width: i32,
    pub height: i32,
    pub url: String,
    pub profile_id: IdType
}

impl FromRow<'_, PgRow> for Figure {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get("figure_id")
            .or_else(|_| row.try_get("id"))?;
        let title: String = row.try_get("title")?;
        let description: Option<String> = row.try_get("description")?;
        let width: i32 = row.try_get("width")?;
        let height: i32 = row.try_get("height")?;
        let url: String = row.try_get("url")?;
        let profile_id: IdType = row.try_get("profile_id")?;

        Ok(Figure {
            id,
            title,
            description,
            width,
            height,
            url,
            profile_id
        })
    }
}