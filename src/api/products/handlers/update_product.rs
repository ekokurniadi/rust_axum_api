use crate::api::products::service::IProductService;
use crate::task::event::{ActionType, Event};
use crate::task::message::Message;
use crate::{
    api::products::models::{
        product_model::ProductModel, product_request_model::ProductModelRequest,
    },
    shared::{api_response::ApiResponse, error::Error},
    state::AppState,
};
use axum::extract::Path;
use axum::{extract::State, Json};
use chrono::Utc;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<ProductModelRequest>,
) -> Result<Json<ApiResponse<ProductModel>>, Error> {
    // return SqlxError::RowNotFound if record not found on database
    state.product_service.get_product_by_id(id).await?;

    let product = state.product_service.update_product(id, &payload).await;

    match product {
        Ok(res) => {
            let _ = state
                .rabbitmq
                .publish_event(Message {
                    action: ActionType::UPDATE.to_string(),
                    push_type: Event::Products.to_string(),
                    message_time: Utc::now().to_rfc3339(),
                    message_uid: Uuid::new_v4().to_string(),
                    payload: res.clone(),
                })
                .await;

            Ok(Json(ApiResponse::new(
                true,
                Some(res),
                "update product success!".to_string(),
            )))
        }
        Err(e) => Err(Error::SqlxError(e)),
    }
}
