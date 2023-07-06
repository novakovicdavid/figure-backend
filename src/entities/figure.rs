use std::fmt::{Display, Formatter};
use serde::{Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::entities::types::IdType;

#[derive(Serialize, Debug)]
pub struct Figure {
    pub id: IdType,
    pub title: String,
    pub description: Option<String>,
    pub width: i32,
    pub height: i32,
    pub url: String,
    pub profile_id: IdType,
}

pub enum FigureDef {
    Table,
    Id,
    Title,
    Description,
    Width,
    Height,
    Url,
    ProfileId,
}

impl FigureDef {
    pub fn as_str(&self) -> &str {
        match self {
            FigureDef::Table => "figures",
            FigureDef::Id => "id",
            FigureDef::Title => "title",
            FigureDef::Description => "description",
            FigureDef::Width => "width",
            FigureDef::Height => "height",
            FigureDef::Url => "url",
            FigureDef::ProfileId => "profile_id",
        }
    }

    pub fn as_table_str(&self) -> &str {
        match self {
            FigureDef::Table => "figures",
            FigureDef::Id => "figures.id",
            FigureDef::Title => "figures.title",
            FigureDef::Description => "figures.description",
            FigureDef::Width => "figures.width",
            FigureDef::Height => "figures.height",
            FigureDef::Url => "figures.url",
            FigureDef::ProfileId => "figures.profile_id",
        }
    }

    pub fn unique(&self) -> &str {
        match self {
            FigureDef::Id => "figure_id",
            _ => self.as_table_str(),
        }
    }
}

impl Display for FigureDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.as_table_str())
    }
}

impl FromRow<'_, PgRow> for Figure {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get(FigureDef::Id.unique())
            .or_else(|_| row.try_get(FigureDef::Id.as_str()))?;
        let title: String = row.try_get(FigureDef::Title.as_str())?;
        let description: Option<String> = row.try_get(FigureDef::Description.as_str())?;
        let width: i32 = row.try_get(FigureDef::Width.as_str())?;
        let height: i32 = row.try_get(FigureDef::Height.as_str())?;
        let url: String = row.try_get(FigureDef::Url.as_str())?;
        let profile_id: IdType = row.try_get(FigureDef::ProfileId.as_str())?;

        Ok(Figure {
            id,
            title,
            description,
            width,
            height,
            url,
            profile_id,
        })
    }
}