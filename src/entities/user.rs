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

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserAndProfileFromQuery {
    #[sqlx(flatten)]
    pub user: User,
    #[sqlx(flatten)]
    pub profile: Profile,
}
