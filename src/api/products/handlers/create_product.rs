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
use axum::{extract::State, Json};
use chrono::Utc;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Json(payload): Json<ProductModelRequest>,
) -> Result<Json<ApiResponse<ProductModel>>, Error> {
    let product = state.product_service.create_new(&payload).await;

    match product {
        Ok(res) => {
            let _ = state
                .rabbitmq
                .publish_event(Message {
                    action: ActionType::INSERT.to_string(),
                    push_type: Event::Products.to_string(),
                    message_time: Utc::now().to_rfc3339(),
                    message_uid: Uuid::new_v4().to_string(),
                    payload: res.clone(),
                })
                .await;

            Ok(Json(ApiResponse::new(
                true,
                Some(res),
                "create product success!".to_string(),
            )))
        }
        Err(e) => Err(Error::SqlxError(e)),
    }
}
