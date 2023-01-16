use serde::Serialize;
use serde_json::json;
use crate::entities::types::IdType;
use crate::entities::user::User;

#[derive(Serialize, Debug)]
pub struct UserDTO {
    pub email: String,
    pub role: String,
    pub id: IdType,
}

impl UserDTO {
    pub fn _to_json(&self) -> String {
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