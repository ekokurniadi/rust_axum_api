use crate::{config::rabbitmq::RabbitMQ, middleware as mw, state::AppState};
use auth_routes::auth_routes;
use axum::{
    http::{header::HeaderValue, method::Method},
    middleware,
    routing::{get, IntoMakeService, Router},
    Json,
};
use category_routes::category_routes;
use product_routes::product_routes;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use user_routes::user_routes;
mod auth_routes;
mod category_routes;
mod product_routes;
mod user_routes;

pub fn create_routes(
    db: &PgPool,
    rabbitmq: &RabbitMQ,
    origin_url: &str,
) -> IntoMakeService<Router> {
    let state = AppState::new(db.clone(), rabbitmq);
    let api_routes = {
        Router::new()
            .merge(auth_routes().with_state(state.clone()))
            .merge(api_v1_routes(origin_url, state.clone()).with_state(state))
    };

    Router::new()
        .merge(health_check_routes())
        .nest("/api/v1", api_routes)
        .into_make_service()
}

fn api_v1_routes(origin_url: &str, app_state: AppState) -> Router<AppState> {
    let cors_layer = CorsLayer::new()
        .allow_origin(vec![origin_url.parse::<HeaderValue>().unwrap()])
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
        .nest("/users", user_routes())
        .layer(ServiceBuilder::new().layer(cors_layer))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(app_state, mw::auth)))
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
