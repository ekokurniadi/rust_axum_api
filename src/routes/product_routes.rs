use crate::{
    api::products::handlers::{
        create_product, delete_product, get_product_by_id, get_products, update_product,
    },
    state::AppState,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn product_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_product::execute))
        .route("/", get(get_products::execute))
        .route("/:id", get(get_product_by_id::execute))
        .route("/:id", delete(delete_product::execute))
        .route("/:id", put(update_product::execute))
}
