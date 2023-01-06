use serde::{Serialize};
use crate::entities::types::Id;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Profile {
    #[sqlx(flatten)]
    pub id: Id,
    pub username: String,
    pub display_name: Option<String>,
    #[sqlx(flatten)]
    pub user_id: Id,
}

#[derive(Serialize, Debug)]
pub struct ProfileDTO {
    pub id: Id,
    pub username: String,
    pub display_name: Option<String>,
}

impl From<Profile> for ProfileDTO {
    fn from(profile: Profile) -> Self {
        Self {
            id: profile.id,
            username: profile.username,
            display_name: profile.display_name,
        }
    }
}