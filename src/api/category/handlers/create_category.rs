use crate::api::category::models::category_model::CategoryModel;
use crate::api::category::models::category_model_request::CategoryModelRequest;
use crate::api::category::service::ICategoryService;
use crate::task::event::{ActionType, Event};
use crate::task::message::Message;
use crate::{
    shared::{api_response::ApiResponse, error::Error},
    state::AppState,
};
use axum::{extract::State, Json};
use chrono::Utc;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Json(payload): Json<CategoryModelRequest>,
) -> Result<Json<ApiResponse<CategoryModel>>, Error> {
    let category = state.category_service.create_new(&payload).await;
    match category {
        Ok(res) => {
            let _ = state
                .rabbitmq
                .publish_event(Message {
                    action: ActionType::INSERT.to_string(),
                    push_type: Event::Category.to_string(),
                    message_time: Utc::now().to_rfc3339(),
                    message_uid: Uuid::new_v4().to_string(),
                    payload: res.clone(),
                })
                .await;

            Ok(Json(ApiResponse::new(
                true,
                Some(res),
                "create category success!".to_string(),
            )))
        }
        Err(e) => Err(Error::SqlxError(e)),
    }
}