use serde::Serialize;
use crate::entities::types::Id;
use crate::entities::user::User;

#[derive(Serialize, Debug)]
pub struct UserDTO {
    pub email: String,
    pub role: String,
    pub id: Id,
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