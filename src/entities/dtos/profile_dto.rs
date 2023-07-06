use serde::Serialize;
use serde_json::json;
use crate::entities::profile::Profile;
use crate::entities::types::IdType;

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
            id: profile.id,
            username: profile.username,
            display_name: profile.display_name,
        }
    }
}

impl From<Profile> for ProfileWithoutUserIdDTO {
    fn from(profile: Profile) -> Self {
        Self {
            id: profile.id,
            username: profile.username,
            display_name: profile.display_name,
            bio: profile.bio,
            banner: profile.banner,
            profile_picture: profile.profile_picture,

        }
    }
}