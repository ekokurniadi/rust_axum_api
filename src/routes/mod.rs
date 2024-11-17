use crate::{config::rabbitmq::RabbitMQ, state::AppState};
use axum::{
    routing::{get, IntoMakeService, Router},
    Json,
};
use category_routes::category_routes;
use product_routes::product_routes;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
mod category_routes;
mod product_routes;
use http::{header::HeaderValue, method::Method};

pub fn create_routes(db: &PgPool, rabbitmq: &RabbitMQ, host: String) -> IntoMakeService<Router> {
    let state = AppState::new(db.clone(), rabbitmq);
    let api_routes = { Router::new().merge(api_v1_routes(host).with_state(state)) };

    Router::new()
        .merge(health_check_routes())
        .nest("/api/v1", api_routes)
        .into_make_service()
}

fn api_v1_routes(host: String) -> Router<AppState> {
    let cors_layer = CorsLayer::new()
        .allow_origin(vec![host.parse::<HeaderValue>().unwrap()])
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
        ]);

    Router::new()
        .nest("/category", category_routes())
        .nest("/products", product_routes())
        .layer(ServiceBuilder::new().layer(cors_layer))
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
