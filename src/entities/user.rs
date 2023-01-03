use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    pub role: String,
    pub id: i32,
}

#[derive(Serialize, Debug)]
pub struct UserDTO {
    pub email: String,
    pub role: String,
    pub id: i32,
}