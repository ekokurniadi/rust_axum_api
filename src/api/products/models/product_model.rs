use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct ProductModel {
    pub id: i32,
    pub name: String,
    pub category_id: i32,
    pub created_at: Option<NaiveDateTime>,
}
