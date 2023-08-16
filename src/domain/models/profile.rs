use lazy_static::lazy_static;
use serde::Serialize;
use unicode_segmentation::UnicodeSegmentation;
use crate::domain::models::types::IdType;
use regex::Regex;
use tracing::warn;
use crate::server_errors::ServerError;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Profile {
    id: IdType,
    username: String,
    display_name: Option<String>,
    bio: Option<String>,
    banner: Option<String>,
    profile_picture: Option<String>,
    user_id: IdType,
}

impl Profile {
    pub fn new(id: IdType, username: String, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>, user_id: IdType) -> Result<Self, ServerError> {
        validate_username(&username)?;

        Ok(Self::new_raw(id, username, display_name, bio, banner, profile_picture, user_id))
    }

    pub fn new_raw(id: IdType, username: String, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>, user_id: IdType) -> Self {
        Self {
            id,
            username,
            display_name,
            bio,
            banner,
            profile_picture,
            user_id,
        }
    }

    pub fn get_id(&self) -> IdType {
        self.id
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_display_name(&self) -> &Option<String> {
        &self.display_name
    }

    pub fn get_bio(&self) -> &Option<String> {
        &self.bio
    }

    pub fn get_banner(&self) -> &Option<String> {
        &self.banner
    }

    pub fn get_profile_picture(&self) -> &Option<String> {
        &self.profile_picture
    }

    pub fn get_user_id(&self) -> IdType {
        self.user_id
    }

    pub fn set_id(&mut self, id: IdType) {
        self.id = id;
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_display_name(&mut self, display_name: Option<String>) {
        self.display_name = display_name
    }

    pub fn set_bio(&mut self, bio: Option<String>) {
        self.bio = bio;
    }

    pub fn set_banner(&mut self, banner: Option<String>) {
        self.banner = banner;
    }

    pub fn set_profile_picture(&mut self, profile_picture: Option<String>) {
        self.profile_picture = profile_picture;
    }

    pub fn set_user_id(&mut self, user_id: IdType) {
        self.user_id = user_id;
    }
}

// Valid username test
// (alphanumerical, optionally a dash surrounded by alphanumerical characters, 15 character limit)
fn validate_username(username: &str) -> Result<(), ServerError> {
    let username_count = username.graphemes(true).count();

    if !USERNAME_REGEX.is_match(username) || !(3..=15).contains(&username_count) {
        return Err(ServerError::InvalidUsername);
    }

    Ok(())
}

lazy_static! {
    static ref USERNAME_REGEX: Regex =
    Regex::new("^[a-zA-Z0-9]+-*[a-zA-Z0-9]+?$").unwrap();
}