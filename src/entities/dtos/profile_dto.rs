use serde::Serialize;
use crate::entities::profile::Profile;
use crate::entities::types::Id;

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