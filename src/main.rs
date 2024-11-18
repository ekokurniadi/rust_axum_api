use config::{db, envi, rabbitmq::RabbitMQ};
use lapin::options::ConfirmSelectOptions;

pub mod api;
pub mod config;
pub mod middleware;
pub mod routes;
pub mod shared;
pub mod state;
pub mod task;
pub mod utils;

#[tokio::main]
async fn main() {
    envi::init_env();
    let database_connection = db::init().await.unwrap();
    let rabbitmq = RabbitMQ::connect().await.unwrap();
    let host = envi::get("HOST", "0.0.0.0".to_string());
    let port = envi::get("POST", "3100".to_string());

    let _ = rabbitmq
        .channel
        .confirm_select(ConfirmSelectOptions::default())
        .await;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let origin_url = format!("{host}:{port}");

    let app = routes::create_routes(&database_connection, &rabbitmq, &origin_url);
    let listener = tokio::net::TcpListener::bind(origin_url).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
