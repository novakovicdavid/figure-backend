use std::fmt::{Display, Formatter};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::domain::models::types::IdType;
use crate::domain::models::user::User;

pub enum UserDef {
    Table,
    Id,
    Email,
    Password,
    Role,
}

impl UserDef {
    pub fn as_str(&self) -> &str {
        match self {
            UserDef::Table => "\"user\"",
            UserDef::Id => "id",
            UserDef::Email => "email",
            UserDef::Password => "password",
            UserDef::Role => "role",
        }
    }

    pub fn as_table_str(&self) -> &str {
        match self {
            UserDef::Table => "\"user\"",
            UserDef::Id => "\"user\".id",
            UserDef::Email => "\"user\".email",
            UserDef::Password => "\"user\".password",
            UserDef::Role => "\"user\".role",
        }
    }

    pub fn unique(&self) -> &str {
        match self {
            UserDef::Id => "user_id",
            _ => self.as_table_str(),
        }
    }
}

impl Display for UserDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.as_table_str())
    }
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get(UserDef::Id.unique())
            .or_else(|_| row.try_get(UserDef::Id.as_str()))?;
        let email: String = row.try_get(UserDef::Email.as_str())?;
        let password: String = row.try_get(UserDef::Password.as_str())?;
        let role: String = row.try_get(UserDef::Role.as_str())?;

        Ok(User::new_raw(id, email, password, role))
    }
}