use serde::{Serialize};
use crate::entities::profile::Profile;
use crate::entities::types::IdType;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub id: IdType,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserAndProfileFromQuery {
    #[sqlx(flatten)]
    pub user: User,
    #[sqlx(flatten)]
    pub profile: Profile,
}
