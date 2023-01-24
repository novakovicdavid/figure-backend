use serde::Serialize;
use std::fmt::{Display, Formatter};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::entities::types::IdType;

#[derive(Serialize, Debug)]
pub struct Profile {
    pub id: IdType,
    pub username: String,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
    pub user_id: IdType,
}

impl FromRow<'_, PgRow> for Profile {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get("profile_id")
            .or_else(|_| row.try_get("id"))?;
        let username: String = row.try_get("username")?;
        let display_name: Option<String> = row.try_get("display_name")?;
        let user_id: IdType = row.try_get("user_id")?;
        let profile_picture: Option<String> = row.try_get("profile_picture")?;
        let bio: Option<String> = row.try_get("bio")?;

        Ok(Profile {
            id,
            username,
            display_name,
            profile_picture,
            bio,
            user_id,
        })
    }
}

pub enum ProfileDef {
    Table,
    Id
}

impl Display for ProfileDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ProfileDef::Table => "profiles",
            ProfileDef::Id => "id"
        };
        write!(f, "{}", message)
    }
}