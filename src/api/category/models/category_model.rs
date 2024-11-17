use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct CategoryModel {
    pub id: i32,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
}
