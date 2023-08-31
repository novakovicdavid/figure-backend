use std::fmt::{Display, Formatter};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::domain::models::profile::Profile;
use crate::domain::models::types::IdType;

pub struct ProfileDef;

impl ProfileDef {
    pub const TABLE: &'static str = "profile";

    pub const ID: &'static str = "profile.id";
    pub const ID_UNIQUE: &'static str = "u_profile_id";

    pub const USERNAME: &'static str = "profile.username";
    pub const DISPLAY_NAME: &'static str = "profile.display_name";
    pub const BIO: &'static str = "profile.bio";
    pub const BANNER: &'static str = "profile.banner";
    pub const PROFILE_PICTURE: &'static str = "profile.profile_picture";

    pub const USER_ID: &'static str = "profile.user_id";
}

impl FromRow<'_, PgRow> for Profile {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get(ProfileDef::ID_UNIQUE)
            .or_else(|_| row.try_get(&ProfileDef::ID[8..]))?;
        let username: String = row.try_get(&ProfileDef::USERNAME[8..])?;
        let display_name: Option<String> = row.try_get(&ProfileDef::DISPLAY_NAME[8..])?;
        let user_id: IdType = row.try_get(&ProfileDef::USER_ID[8..])?;
        let profile_picture: Option<String> = row.try_get(&ProfileDef::PROFILE_PICTURE[8..])?;
        let bio: Option<String> = row.try_get(&ProfileDef::BIO[8..])?;
        let banner: Option<String> = row.try_get(&ProfileDef::BANNER[8..])?;

        Ok(Profile::new_raw(
            id,
            username,
            display_name,
            bio,
            banner,
            profile_picture,
            user_id,
        ))
    }
}