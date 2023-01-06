use serde::{Serialize};
use crate::entities::profile::Profile;
use crate::entities::types::Id;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct User {
    #[sqlx(flatten)]
    pub id: Id,
    pub email: String,
    pub password: String,
    pub role: String,
}

pub enum UserDef {
    Table,
}

// pub enum UserDef {
//     Id,
//     Email,
//     Password,
//     Role,
// }

#[allow(dead_code)]
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserAndProfile {
    #[sqlx(flatten)]
    pub user: User,
    #[sqlx(flatten)]
    pub profile: Profile,
}

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