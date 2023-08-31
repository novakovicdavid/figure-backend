use std::fmt::{Debug, Display, Formatter};
use sqlx::{ColumnIndex, Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::domain::models::figure::Figure;
use crate::domain::models::types::IdType;

pub struct FigureDef;

impl FigureDef {
    pub const TABLE: &'static str = "figure";

    pub const ID: &'static str = "figure.id";
    pub const ID_UNIQUE: &'static str = "u_figure_id";

    pub const TITLE: &'static str = "figure.title";
    pub const DESCRIPTION: &'static str = "figure.description";
    pub const WIDTH: &'static str = "figure.width";
    pub const HEIGHT: &'static str = "figure.height";
    pub const URL: &'static str = "figure.url";

    pub const PROFILE_ID: &'static str = "figure.profile_id";
}

impl FromRow<'_, PgRow> for Figure {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get(FigureDef::ID_UNIQUE)
            .or_else(|_| row.try_get(&FigureDef::ID[7..]))?;
        let title: String = row.try_get(&FigureDef::TITLE[7..])?;
        let description: Option<String> = row.try_get(&FigureDef::DESCRIPTION[7..])?;
        let width: i32 = row.try_get(&FigureDef::WIDTH[7..])?;
        let height: i32 = row.try_get(&FigureDef::HEIGHT[7..])?;
        let url: String = row.try_get(&FigureDef::URL[7..])?;
        let profile_id: IdType = row.try_get(&FigureDef::PROFILE_ID[7..])?;

        Ok(Figure::new_raw(
            id,
            title,
            description,
            width,
            height,
            url,
            profile_id,
        ))
    }
}

