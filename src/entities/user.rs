use serde::{Serialize};
use sea_query::enum_def;

#[allow(dead_code)]
#[enum_def(suffix = "Def")]
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub email: String,
    pub password: String,
    pub role: String,
    pub id: i64,
}

#[derive(Serialize, Debug)]
pub struct UserDTO {
    pub email: String,
    pub role: String,
    pub id: i64,
}