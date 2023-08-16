use serde::{Serialize};
use crate::domain::models::types::IdType;

#[derive(Serialize, Debug)]
pub struct Figure {
    pub id: IdType,
    pub title: String,
    pub description: Option<String>,
    pub width: i32,
    pub height: i32,
    pub url: String,
    pub profile_id: IdType,
}