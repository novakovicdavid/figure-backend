use serde::{Serialize};
use crate::utilities::types::IdType;
use crate::server_errors::ServerError;

#[derive(Serialize, Debug)]
pub struct Figure {
    id: IdType,
    title: String,
    description: Option<String>,
    width: i32,
    height: i32,
    url: String,
    profile_id: IdType,
}

impl Figure {
    pub fn new(id: IdType,
               title: String,
               description: Option<String>,
               width: i32,
               height: i32,
               url: String,
               profile_id: IdType) -> Result<Self, ServerError> {
        Self::check_size(width as u32, height as u32)?;

        Ok(Self {
            id,
            title,
            description,
            width,
            height,
            url,
            profile_id,
        })
    }

    pub fn new_raw(id: IdType,
                   title: String,
                   description: Option<String>,
                   width: i32,
                   height: i32,
                   url: String,
                   profile_id: IdType) -> Self {
        Self {
            id,
            title,
            description,
            width,
            height,
            url,
            profile_id,
        }
    }

    pub fn check_size_i32(width: i32, height: i32) -> Result<(), ServerError> {
        Self::check_size(width as u32, height as u32)
    }

    pub fn check_size(width: u32, height: u32) -> Result<(), ServerError> {
        if width + height > 6000 {
            return Err(ServerError::ImageDimensionsTooLarge);
        }
        Ok(())
    }

    pub fn get_id(&self) -> IdType {
        self.id
    }

    pub fn set_id(&mut self, id: IdType) {
        self.id = id;
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn get_profile_id(&self) -> IdType {
        self.profile_id
    }

    pub fn set_profile_id(&mut self, profile_id: IdType) {
        self.profile_id = profile_id;
    }
}