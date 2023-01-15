use serde::Serialize;
use serde_json::json;
use crate::entities::profile::Profile;
use crate::entities::types::IdType;

#[derive(Serialize, Debug)]
pub struct ProfileDTO {
    pub id: IdType,
    pub username: String,
    pub display_name: Option<String>,
}

impl ProfileDTO {
    pub fn to_json(&self) -> String {
        json!({
            "profile": &self
        }).to_string()
    }
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