use serde::{Serialize};
use sea_query::enum_def;
use crate::entities::types::Id;

#[allow(dead_code)]
#[enum_def(suffix = "Def")]
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Figure {
    pub id: Id,
    pub title: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub profile_id: Id,
}