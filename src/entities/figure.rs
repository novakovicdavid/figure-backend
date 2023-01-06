use std::fmt::{Display, Formatter};
use serde::{Serialize};
use crate::entities::types::Id;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Figure {
    #[sqlx(flatten)]
    pub id: Id,
    pub title: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    #[sqlx(flatten)]
    pub profile_id: Id,
}

pub enum FigureDef {
    Table,
    Id
}

impl Display for FigureDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            FigureDef::Table => "figures",
            FigureDef::Id => "id"
        };
        write!(f, "{}", message)
    }
}