use crate::api::products::service::IProductService;
use crate::shared::api_response::{ApiResponseWithPagination, Meta, RequestPaginationParam};
use crate::{
    api::products::models::product_model::ProductModel, shared::error::Error, state::AppState,
};
use axum::extract::Query;
use axum::{extract::State, Json};

#[axum::debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Query(params): Query<RequestPaginationParam>,
) -> Result<Json<ApiResponseWithPagination<ProductModel>>, Error> {
    let (product, total) = state
        .product_service
        .get_products(params.page, params.limit)
        .await;

    match product {
        Ok(res) => {
            let meta = Meta {
                total_data: total,
                page: params.page,
                per_page: params.limit,
            };

            let response = ApiResponseWithPagination {
                status: true,
                data: res,
                meta,
                message: "get products success!".to_string(),
            };

            Ok(Json(response))
        }
        Err(e) => Err(Error::SqlxError(e)),
    }
}
