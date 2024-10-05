use crate::model::time::DateTime;

#[derive(sqlx::FromRow)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub expires: DateTime,
}
