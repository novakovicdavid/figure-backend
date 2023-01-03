use serde::{Serialize};
use sea_query::enum_def;

#[allow(dead_code)]
#[enum_def(suffix = "Def")]
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Profile {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub user_id: i64,
}

#[derive(Serialize, Debug)]
pub struct ProfileDTO {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
}

impl Profile {
    pub fn into_dto(self) -> ProfileDTO {
        ProfileDTO {
            id: self.id,
            username: self.username,
            display_name: self.display_name
        }
    }
}