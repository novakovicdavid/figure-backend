use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Figure {
    pub id: i32,
    pub title: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub profile_id: i32,
}