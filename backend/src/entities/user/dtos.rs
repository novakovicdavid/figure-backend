use serde::Serialize;
use serde_json::json;
use crate::utilities::types::IdType;
use crate::entities::user::user::User;

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
            email: user.get_email().to_string(),
            role: user.get_role().to_string(),
            id: user.get_id(),
        }
    }
}