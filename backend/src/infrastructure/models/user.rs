use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::domain::models::types::IdType;
use crate::domain::models::user::User;

pub struct UserDef;

impl UserDef {
    pub const TABLE: &'static str = "\"user\"";

    pub const ID: &'static str = "\"user\".id";
    pub const ID_UNIQUE: &'static str = "u_user_id";

    pub const EMAIL: &'static str = "\"user\".email";
    pub const PASSWORD: &'static str = "\"user\".password";
    pub const ROLE: &'static str = "\"user\".role";

    pub fn with_table(column: &str) -> String {
        format!("{}.{}", Self::TABLE, column)
    }
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get(UserDef::ID_UNIQUE)
            .or_else(|_| row.try_get(&UserDef::ID[7..]))?;
        let email: String = row.try_get(&UserDef::EMAIL[7..])?;
        let password: String = row.try_get(&UserDef::PASSWORD[7..])?;
        let role: String = row.try_get(&UserDef::ROLE[7..])?;

        Ok(User::new_raw(id, email, password, role))
    }
}