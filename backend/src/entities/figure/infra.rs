use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::domain::figure::Figure;
use crate::utilities::types::IdType;

impl FromRow<'_, PgRow> for Figure {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get("u_figure_id")
            .or_else(|_| row.try_get("id"))?;
        let title: String = row.try_get("title")?;
        let description: Option<String> = row.try_get("description")?;
        let width: i32 = row.try_get("width")?;
        let height: i32 = row.try_get("height")?;
        let url: String = row.try_get("url")?;
        let profile_id: IdType = row.try_get("profile_id")?;

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

