use serde::Serialize;
use serde_json::json;
use crate::domain::models::profile::Profile;
use crate::domain::models::types::IdType;

#[derive(Serialize, Debug, PartialEq)]
pub struct ProfileDTO {
    pub id: IdType,
    pub username: String,
    pub display_name: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ProfileWithoutUserIdDTO {
    pub id: IdType,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub banner: Option<String>,
    pub profile_picture: Option<String>,
}

impl ProfileDTO {
    pub fn to_json(&self) -> String {
        to_json(&self)
    }
}

impl ProfileWithoutUserIdDTO {
    pub fn to_json(&self) -> String {
        to_json(&self)
    }
}

pub fn to_json(profile: &impl Serialize) -> String {
    json!({
            "profile": profile
        }).to_string()
}

impl From<Profile> for ProfileDTO {
    fn from(profile: Profile) -> Self {
        Self {
            id: profile.get_id(),
            username: profile.get_username().to_string(),
            display_name: profile.get_display_name().clone(),
        }
    }
}

impl From<Profile> for ProfileWithoutUserIdDTO {
    fn from(profile: Profile) -> Self {
        Self {
            id: profile.get_id(),
            username: profile.get_username().to_string(),
            display_name: profile.get_display_name().clone(),
            bio: profile.get_bio().clone(),
            banner: profile.get_banner().clone(),
            profile_picture: profile.get_profile_picture().clone(),

        }
    }
}