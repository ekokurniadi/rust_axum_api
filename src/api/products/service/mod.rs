use sqlx::Error;
use std::sync::Arc;

use super::{
    models::product_model::ProductModel,
    models::product_request_model::ProductModelRequest,
    repository::{IProductRepository, ProductRepository},
};

pub trait IProductService: Send + Sync {
    fn new(product_repository: Arc<ProductRepository>) -> Self;

    fn create_new(
        &self,
        product: &ProductModelRequest,
    ) -> impl std::future::Future<Output = Result<ProductModel, Error>> + Send;

    fn get_products(
        &self,
        page: i64,
        limit: i64,
    ) -> impl std::future::Future<Output = (Result<Vec<ProductModel>, Error>, i64)> + Send;

    fn get_product_by_id(
        &self,
        id: i32,
    ) -> impl std::future::Future<Output = Result<ProductModel, Error>> + Send;

    fn update_product(
        &self,
        product_id: i32,
        product: &ProductModelRequest,
    ) -> impl std::future::Future<Output = Result<ProductModel, Error>> + Send;

    fn delete_product(
        &self,
        product_id: i32,
    ) -> impl std::future::Future<Output = Result<bool, Error>> + Send;
}

#[derive(Clone, Debug)]
pub struct ProductService {
    product_repository: Arc<ProductRepository>,
}

impl IProductService for ProductService {
    fn new(product_repository: Arc<ProductRepository>) -> Self {
        Self { product_repository }
    }

    async fn create_new(&self, product: &ProductModelRequest) -> Result<ProductModel, Error> {
        self.product_repository.create_product(product).await
    }

    async fn get_products(&self, page: i64, limit: i64) -> (Result<Vec<ProductModel>, Error>, i64) {
        self.product_repository.get_products(page, limit).await
    }

    async fn get_product_by_id(&self, id: i32) -> Result<ProductModel, Error> {
        self.product_repository.get_product_by_id(id).await
    }

    async fn update_product(
        &self,
        product_id: i32,
        product: &ProductModelRequest,
    ) -> Result<ProductModel, Error> {
        self.product_repository
            .update_product(product_id, product)
            .await
    }

    async fn delete_product(&self, product_id: i32) -> Result<bool, Error> {
        self.product_repository.delete_product(product_id).await
    }
}
