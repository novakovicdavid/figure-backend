use serde::{Serialize, Deserialize};
use sqlx::{Error, FromRow, Row, Type};
use sqlx::postgres::PgRow;

#[derive(Serialize, Deserialize, Type, Clone, Copy, Debug)]
pub struct Id(pub IdType);
pub type IdType = i64;

impl FromRow<'_, PgRow> for Id {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id_result: Result<IdType, Error> = row
            .try_get("user_id")
            .or_else(|_| row.try_get("profile_id"))
            .or_else(|_| row.try_get("id"));
        match id_result {
            Ok(id) => Ok(Id(id)),
            Err(e) => Err(e)
        }
    }
}