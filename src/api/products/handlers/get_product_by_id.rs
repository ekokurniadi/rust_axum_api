use crate::api::products::service::IProductService;
use crate::shared::api_response::ApiResponse;
use crate::{
    api::products::models::product_model::ProductModel, shared::error::Error, state::AppState,
};
use axum::extract::Path;
use axum::{extract::State, Json};

#[axum::debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Path(params): Path<i32>,
) -> Result<Json<ApiResponse<ProductModel>>, Error> {
    let product = state.product_service.get_product_by_id(params).await;

    match product {
        Ok(res) => {
            let response = ApiResponse {
                status: true,
                data: Some(res),
                message: "get products success".to_string(),
            };

            Ok(Json(response))
        }
        Err(e) => Err(Error::SqlxError(e)),
    }
}
