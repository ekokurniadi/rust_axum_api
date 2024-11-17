use crate::{config::rabbitmq::RabbitMQ, state::AppState};
use axum::{
    routing::{get, IntoMakeService, Router},
    Json,
};
use product_routes::product_routes;
use sqlx::PgPool;
mod product_routes;

pub fn create_routes(db: &PgPool, rabbitmq: &RabbitMQ) -> IntoMakeService<Router> {
    let state = AppState::new(db.clone(), rabbitmq);
    let api_routes = { Router::new().merge(api_v1_routes().with_state(state)) };

    Router::new()
        .merge(health_check_routes())
        .nest("/api/v1", api_routes)
        .into_make_service()
}

fn api_v1_routes() -> Router<AppState> {
    Router::new().nest("/products", product_routes())
}

fn health_check_routes() -> Router {
    Router::new().route(
        "/health_check",
        get(|| async {
            let response = serde_json::json!({
                "status":true,
                "message": "api running"
            });

            Json(response)
        }),
    )
}
