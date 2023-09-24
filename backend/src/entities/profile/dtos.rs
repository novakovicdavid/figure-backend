use serde::Serialize;
use crate::entities::profile::model::Profile;
use crate::utilities::types::IdType;
use derive_name::with_name;


#[derive(Serialize, Debug, PartialEq)]
#[with_name(profile)]
pub struct ProfileDTO {
    pub id: IdType,
    pub username: String,
    pub display_name: Option<String>,
}

#[derive(Serialize, Debug)]
#[with_name(profile)]
pub struct ProfileWithoutUserIdDTO {
    pub id: IdType,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub banner: Option<String>,
    pub profile_picture: Option<String>,
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