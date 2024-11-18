use crate::api::users::models::user_model::UserModelResponse;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "email is not valid"))]
    #[validate(length(min = 1, message = "email is required"))]
    pub email: String,
    #[validate(length(min = 3, message = "password must be between 3 and 20 characters"))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub user: UserModelResponse,
    pub token: String,
}
