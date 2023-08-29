use std::fmt::{Display, Formatter};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::domain::models::profile::Profile;
use crate::domain::models::types::IdType;

pub struct ProfileDef(&'static str);

impl ProfileDef {
    pub const TABLE: &'static str = "profile";

    pub const ID: &'static str = "id";
    pub const ID_UNIQUE: &'static str = "profile_id";

    pub const USERNAME: &'static str = "username";
    pub const DISPLAY_NAME: &'static str = "display_name";
    pub const BIO: &'static str = "bio";
    pub const BANNER: &'static str = "banner";
    pub const PROFILE_PICTURE: &'static str = "profile_picture";

    pub const USER_ID: &'static str = "user_id";

    // pub fn with_table(column: Self) -> String {
    //     format!("{}.{}", Self::TABLE, column)
    // }
}

impl Display for ProfileDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl FromRow<'_, PgRow> for Profile {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get(ProfileDef::ID_UNIQUE)
            .or_else(|_| row.try_get(ProfileDef::ID))?;
        let username: String = row.try_get(ProfileDef::USERNAME)?;
        let display_name: Option<String> = row.try_get(ProfileDef::DISPLAY_NAME)?;
        let user_id: IdType = row.try_get(ProfileDef::USER_ID)?;
        let profile_picture: Option<String> = row.try_get(ProfileDef::PROFILE_PICTURE)?;
        let bio: Option<String> = row.try_get(ProfileDef::BIO)?;
        let banner: Option<String> = row.try_get(ProfileDef::BANNER)?;

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