use std::fmt::{Display, Formatter};
use serde::{Serialize};
use serde_json::json;
use crate::entities::dtos::profile_dto::ProfileDTO;
use crate::entities::types::Id;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Figure {
    #[sqlx(flatten)]
    pub id: Id,
    pub title: String,
    pub description: Option<String>,
    pub width: i32,
    pub height: i32,
    pub url: String,
    #[sqlx(flatten)]
    pub profile: ProfileDTO,
}

impl Figure {
    pub fn to_json(&self) -> String {
        json!({
            "figure": &self
        }).to_string()
    }
}