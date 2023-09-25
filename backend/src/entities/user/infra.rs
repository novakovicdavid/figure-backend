use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::utilities::types::IdType;
use crate::entities::user::model::User;

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: IdType = row.try_get("u_user_id")
            .or_else(|_| row.try_get("id"))?;
        let email: String = row.try_get("email")?;
        let password: String = row.try_get("password")?;
        let role: String = row.try_get("role")?;

        Ok(User::new_raw(id, email, password, role))
    }
}