use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Profile {
    pub id: i32,
    pub username: String,
    pub display_name: Option<String>,
    pub user_id: i32,
}

#[derive(Serialize, Debug)]
pub struct ProfileDTO {
    pub id: i32,
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