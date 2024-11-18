use crate::api::users::models::user_model::{UserModelRequest, UserModelResponse};
use crate::api::users::service::IUserService;

use crate::{
    shared::{api_response::ApiResponse, error::Error},
    state::AppState,
};
use axum::{extract::State, Json};

#[axum::debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Json(payload): Json<UserModelRequest>,
) -> Result<Json<ApiResponse<UserModelResponse>>, Error> {
    let user = state.user_service.create_user(&payload).await;

    match user {
        Ok(res) => Ok(Json(ApiResponse::new(
            true,
            Some(res),
            "register user success!".to_string(),
        ))),
        Err(e) => Err(e),
    }
}
