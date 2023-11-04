use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::domain::profile::Profile;
use crate::utilities::types::IdType;

impl FromRow<'_, PgRow> for Profile {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get("u_profile_id")
            .or_else(|_| row.try_get("id"))?;
        let username: String = row.try_get("username")?;
        let display_name: Option<String> = row.try_get("display_name")?;
        let user_id: IdType = row.try_get("user_id")?;
        let profile_picture: Option<String> = row.try_get("profile_picture")?;
        let bio: Option<String> = row.try_get("bio")?;
        let banner: Option<String> = row.try_get("banner")?;

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