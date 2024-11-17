use crate::{
    api::category::handlers::{
        create_category, delete_category, get_categories, get_category_by_id, update_category,
    },
    state::AppState,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn category_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_category::execute))
        .route("/", get(get_categories::execute))
        .route("/:id", get(get_category_by_id::execute))
        .route("/:id", delete(delete_category::execute))
        .route("/:id", put(update_category::execute))
}
