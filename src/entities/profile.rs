use std::fmt::{Display, Formatter};
use serde::Serialize;
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::entities::types::IdType;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Profile {
    pub id: IdType,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub banner: Option<String>,
    pub profile_picture: Option<String>,
    pub user_id: IdType,
}

pub enum ProfileDef {
    Table,
    Id,
    Username,
    DisplayName,
    Bio,
    Banner,
    ProfilePicture,
    UserId,
}

impl ProfileDef {
    pub fn as_str(&self) -> &str {
        match self {
            ProfileDef::Table => "profile",
            ProfileDef::Id => "id",
            ProfileDef::Username => "username",
            ProfileDef::DisplayName => "display_name",
            ProfileDef::Bio => "bio",
            ProfileDef::Banner => "banner",
            ProfileDef::ProfilePicture => "profile_picture",
            ProfileDef::UserId => "user_id",
        }
    }

    pub fn as_table_str(&self) -> &str {
        match self {
            ProfileDef::Table => "profile",
            ProfileDef::Id => "profile.id",
            ProfileDef::Username => "profile.username",
            ProfileDef::DisplayName => "profile.display_name",
            ProfileDef::Bio => "profile.bio",
            ProfileDef::Banner => "profile.banner",
            ProfileDef::ProfilePicture => "profile.profile_picture",
            ProfileDef::UserId => "profile.user_id",
        }
    }

    pub fn unique(&self) -> &str {
        match self {
            ProfileDef::Id => "profile_id",
            _ => self.as_table_str(),
        }
    }
}

impl Display for ProfileDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.as_table_str())
    }
}

impl FromRow<'_, PgRow> for Profile {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get(ProfileDef::Id.unique())
            .or_else(|_| row.try_get(ProfileDef::Id.as_str()))?;
        let username: String = row.try_get(ProfileDef::Username.as_str())?;
        let display_name: Option<String> = row.try_get(ProfileDef::DisplayName.as_str())?;
        let user_id: IdType = row.try_get(ProfileDef::UserId.as_str())?;
        let profile_picture: Option<String> = row.try_get(ProfileDef::ProfilePicture.as_str())?;
        let bio: Option<String> = row.try_get(ProfileDef::Bio.as_str())?;
        let banner: Option<String> = row.try_get(ProfileDef::Banner.as_str())?;

        Ok(Profile {
            id,
            username,
            display_name,
            profile_picture,
            bio,
            user_id,
            banner,
        })
    }
}