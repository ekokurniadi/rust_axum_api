use crate::api::category::models::category_model::CategoryModel;
use crate::api::category::service::ICategoryService;
use crate::shared::api_response::ApiResponse;
use crate::{shared::error::Error, state::AppState};
use axum::extract::Path;
use axum::{extract::State, Json};

#[axum::debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Path(params): Path<i32>,
) -> Result<Json<ApiResponse<CategoryModel>>, Error> {
    let category = state.category_service.get_category_by_id(params).await;

    match category {
        Ok(res) => {
            let response = ApiResponse {
                status: true,
                data: Some(res),
                message: "get category success!".to_string(),
            };

            Ok(Json(response))
        }
        Err(e) => Err(Error::SqlxError(e)),
    }
}
