use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    api::products::{
        repository::{IProductRepository, ProductRepository},
        service::{IProductService, ProductService},
    },
    config::rabbitmq::RabbitMQ,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub product_service: Arc<ProductService>,
    pub rabbitmq: Arc<RabbitMQ>,
}

impl AppState {
    pub fn new(db: PgPool, rabbitmq: &RabbitMQ) -> AppState {
        let product_repository = ProductRepository::new(db.clone());
        let product_service = ProductService::new(Arc::new(product_repository.clone()));
        Self {
            product_service: Arc::new(product_service.clone()),
            rabbitmq: Arc::new(rabbitmq.clone()),
        }
    }
}
