use config::{db, envi, rabbitmq::RabbitMQ};
use lapin::options::ConfirmSelectOptions;
use std::env;

pub mod api;
pub mod config;
pub mod routes;
pub mod shared;
pub mod state;
pub mod task;

#[tokio::main]
async fn main() {
    envi::init_env();
    let database_connection = db::init().await.unwrap();
    let rabbitmq = RabbitMQ::connect().await.unwrap();
    let host = env::var("HOST").expect("HOST MUST BE SET");
    let port = env::var("PORT").expect("PORT MUST BE SET");

    let _ = rabbitmq
        .channel
        .confirm_select(ConfirmSelectOptions::default())
        .await;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = routes::create_routes(&database_connection, &rabbitmq, host.clone());
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
