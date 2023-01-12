use serde::Serialize;
use serde_json::json;
use crate::entities::types::Id;
use crate::entities::user::User;

#[derive(Serialize, Debug)]
pub struct UserDTO {
    pub email: String,
    pub role: String,
    pub id: Id,
}

impl UserDTO {
    pub fn to_json(&self) -> String {
        json!({
            "user": &self
        }).to_string()
    }
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        Self {
            email: user.email,
            role: user.role,
            id: user.id,
        }
    }
}