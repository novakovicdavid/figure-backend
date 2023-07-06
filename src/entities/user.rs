use std::fmt::{Display, Formatter};
use serde::{Serialize};
use crate::entities::profile::Profile;
use crate::entities::types::IdType;

#[derive(Serialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub id: IdType,
    pub email: String,
    pub password: String,
    pub role: String,
}

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
            UserDef::Table => "users",
            UserDef::Id => "id",
            UserDef::Email => "email",
            UserDef::Password => "password",
            UserDef::Role => "role",
        }
    }

    pub fn as_table_str(&self) -> &str {
        match self {
            UserDef::Table => "users",
            UserDef::Id => "users.id",
            UserDef::Email => "users.email",
            UserDef::Password => "users.password",
            UserDef::Role => "users.role",
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



#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserAndProfileFromQuery {
    #[sqlx(flatten)]
    pub user: User,
    #[sqlx(flatten)]
    pub profile: Profile,
}
