use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct UserModelRequest {
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,
    #[validate(email(message = "email is not valid"))]
    #[validate(length(min = 1, message = "email is required"))]
    pub email: String,
    #[validate(length(min = 3, message = "password must be between 3 and 20 characters"))]
    pub password: String,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct UserModelResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
}
