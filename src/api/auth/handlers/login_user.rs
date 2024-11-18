use crate::api::auth::models::{LoginRequest, LoginResponse};
use crate::api::users::service::IUserService;

use crate::{
    shared::{api_response::ApiResponse, error::Error},
    state::AppState,
};
use axum::{extract::State, Json};

#[axum::debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, Error> {
    let user = state.user_service.login(&payload).await;

    match user {
        Ok(res) => Ok(Json(ApiResponse::new(
            true,
            Some(res),
            "login success!".to_string(),
        ))),
        Err(e) => Err(e),
    }
}
