use serde::{Serialize};
use sqlx::{Error, FromRow};
use sqlx::postgres::PgRow;
use tracing::error;
use crate::domain::models::figure::Figure;
use crate::domain::models::profile::Profile;

#[derive(Serialize, Debug)]
pub struct FigureAndProfile {
    figure: Figure,
    profile: Profile
}

impl FigureAndProfile {
    pub fn get_figure_and_profile(self) -> (Figure, Profile) {
        (self.figure, self.profile)
    }
}

impl FromRow<'_, PgRow> for FigureAndProfile {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let figure = Figure::from_row(row)?;
        let profile = Profile::from_row(row)?;

        Ok(FigureAndProfile {
            figure,
            profile,
        })
    }
}