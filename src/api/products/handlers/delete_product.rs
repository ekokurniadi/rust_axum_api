use crate::api::products::service::IProductService;
use crate::shared::api_response::ApiResponse;
use crate::task::event::{ActionType, Event};
use crate::task::message::Message;
use crate::{shared::error::Error, state::AppState};
use axum::extract::Path;
use axum::{extract::State, Json};
use chrono::Utc;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Path(params): Path<i32>,
) -> Result<Json<ApiResponse<i32>>, Error> {
    state.product_service.get_product_by_id(params).await?;

    let result = state.product_service.delete_product(params).await;

    match result {
        Ok(_) => {
            let _ = state
                .rabbitmq
                .publish_event(Message {
                    action: ActionType::DELETE.to_string(),
                    push_type: Event::Products.to_string(),
                    message_time: Utc::now().to_rfc3339(),
                    message_uid: Uuid::new_v4().to_string(),
                    payload: params,
                })
                .await;

            let response = ApiResponse {
                status: true,
                data: None,
                message: "delete products success!".to_string(),
            };

            Ok(Json(response))
        }
        Err(e) => Err(Error::SqlxError(e)),
    }
}
