use std::fmt::{Debug, Display, Formatter};
use sqlx::{ColumnIndex, Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::domain::models::figure::Figure;
use crate::domain::models::types::IdType;

pub struct FigureDef(&'static str);

impl FigureDef {
    pub const TABLE: &'static str = "figure";

    pub const ID: &'static str = "id";
    pub const ID_UNIQUE: &'static str = "figure_id";

    pub const TITLE: &'static str = "title";
    pub const DESCRIPTION: &'static str = "description";
    pub const WIDTH: &'static str = "width";
    pub const HEIGHT: &'static str = "height";
    pub const URL: &'static str = "url";

    pub const PROFILE_ID: &'static str = "profile_id";
}

impl Display for FigureDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl FromRow<'_, PgRow> for Figure {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get(FigureDef::ID_UNIQUE)
            .or_else(|_| row.try_get(FigureDef::ID))?;
        let title: String = row.try_get(FigureDef::TITLE)?;
        let description: Option<String> = row.try_get(FigureDef::DESCRIPTION)?;
        let width: i32 = row.try_get(FigureDef::WIDTH)?;
        let height: i32 = row.try_get(FigureDef::HEIGHT)?;
        let url: String = row.try_get(FigureDef::URL)?;
        let profile_id: IdType = row.try_get(FigureDef::PROFILE_ID)?;

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

impl Debug for FigureDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

