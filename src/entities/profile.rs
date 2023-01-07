use serde::Serialize;
use std::fmt::{Display, Formatter};
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

pub enum ProfileDef {
    Table,
    Id
}

impl Display for ProfileDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ProfileDef::Table => "profiles",
            ProfileDef::Id => "id"
        };
        write!(f, "{}", message)
    }
}