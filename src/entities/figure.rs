use serde::{Serialize};
use sea_query::enum_def;

#[allow(dead_code)]
#[enum_def(suffix = "Def")]
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Figure {
    pub id: i64,
    pub title: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub profile_id: i64,
}