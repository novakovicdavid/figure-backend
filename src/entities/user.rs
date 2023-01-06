use serde::{Serialize};
use sea_query::enum_def;
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::entities::profile::Profile;
use crate::entities::types::Id;

#[allow(dead_code)]
#[enum_def(suffix = "Def")]
#[derive(Serialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    pub role: String,
    pub id: Id,
}

#[allow(dead_code)]
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserAndProfile {
    #[sqlx(flatten)]
    pub user: User,
    #[sqlx(flatten)]
    pub profile: Profile,
}

#[derive(Serialize, Debug)]
pub struct UserDTO {
    pub email: String,
    pub role: String,
    pub id: Id,
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id = row.try_get("user_id").or_else(|_| row.try_get("id"));
        let d: Result<Id, Error> = row.try_get("user_id");
        println!("{:?}", d);
        match id {
            Ok(id) => Ok(Self {
                id,
                email: row.try_get("email")?,
                password: row.try_get("password")?,
                role: row.try_get("role")?
            }),
            Err(e) => Err(e)
        }
    }
}

impl ToString for UserDef {
    fn to_string(&self) -> String {
        match self {
            UserDef::Email => "email".to_string(),

        }
    }
}