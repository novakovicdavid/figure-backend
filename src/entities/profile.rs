use serde::{Serialize};
use crate::entities::types::Id;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Profile {
    #[sqlx(flatten)]
    pub id: Id,
    pub username: String,
    pub display_name: Option<String>,
    #[sqlx(flatten)]
    pub user_id: Id,
}