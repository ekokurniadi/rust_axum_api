use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    api::{
        category::{
            repository::{CategoryRepository, ICategoryRepository},
            service::{CategoryService, ICategoryService},
        },
        products::{
            repository::{IProductRepository, ProductRepository},
            service::{IProductService, ProductService},
        },
    },
    config::rabbitmq::RabbitMQ,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub product_service: Arc<ProductService>,
    pub category_service: Arc<CategoryService>,
    pub rabbitmq: Arc<RabbitMQ>,
}

impl AppState {
    pub fn new(db: PgPool, rabbitmq: &RabbitMQ) -> AppState {
        let product_repository = ProductRepository::new(db.clone());
        let product_service = ProductService::new(Arc::new(product_repository.clone()));

        let category_repository = CategoryRepository::new(db.clone());
        let category_service = CategoryService::new(Arc::new(category_repository.clone()));
        Self {
            product_service: Arc::new(product_service.clone()),
            category_service: Arc::new(category_service.clone()),
            rabbitmq: Arc::new(rabbitmq.clone()),
        }
    }
}
