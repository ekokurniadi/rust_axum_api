use crate::api::category::models::category_model::CategoryModel;
use crate::api::category::service::ICategoryService;
use crate::shared::api_response::{ApiResponseWithPagination, Meta, RequestPaginationParam};
use crate::{shared::error::Error, state::AppState};
use axum::extract::Query;
use axum::{extract::State, Json};

#[axum::debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Query(params): Query<RequestPaginationParam>,
) -> Result<Json<ApiResponseWithPagination<CategoryModel>>, Error> {
    let (categories, total) = state
        .category_service
        .get_categories(params.page, params.limit)
        .await;

    match categories {
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
                message: "get categories success!".to_string(),
            };

            Ok(Json(response))
        }
        Err(e) => Err(Error::SqlxError(e)),
    }
}
