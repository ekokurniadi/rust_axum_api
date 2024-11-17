use super::models::{product_model::ProductModel, product_request_model::ProductModelRequest};
use sqlx::{Error, PgPool, Result};

pub trait IProductRepository: Send + Sync {
    fn new(pool: PgPool) -> Self;

    fn create_product(
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

#[derive(Debug, Clone)]
pub struct ProductRepository(PgPool);

impl IProductRepository for ProductRepository {
    fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    async fn create_product(&self, product: &ProductModelRequest) -> Result<ProductModel, Error> {
        sqlx::query_as::<_, ProductModel>(
            "INSERT INTO products (name,category_id) VALUES ($1,$2) RETURNING *",
        )
        .bind(product.name.clone())
        .bind(product.category_id)
        .fetch_one(&self.0)
        .await
    }

    async fn get_products(&self, page: i64, limit: i64) -> (Result<Vec<ProductModel>, Error>, i64) {
        let offset = (page - 1) * limit;
        let result = sqlx::query_as!(
            ProductModel,
            "SELECT * FROM products order by id LIMIT $1 OFFSET $2",
            limit,
            offset,
        )
        .fetch_all(&self.0)
        .await;

        let total_count = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
            .fetch_one(&self.0)
            .await
            .unwrap_or(Some(0));

        (result, total_count.unwrap_or(0))
    }

    async fn get_product_by_id(&self, id: i32) -> Result<ProductModel, Error> {
        sqlx::query_as!(ProductModel, "SELECT * FROM products where id = $1", id,)
            .fetch_one(&self.0)
            .await
    }

    async fn update_product(
        &self,
        product_id: i32,
        product: &ProductModelRequest,
    ) -> Result<ProductModel, Error> {
        sqlx::query_as::<_, ProductModel>(
            "UPDATE products set name = $1, category_id = $2 where id = $3 RETURNING *",
        )
        .bind(product.name.clone())
        .bind(product.category_id)
        .bind(product_id)
        .fetch_one(&self.0)
        .await
    }

    async fn delete_product(&self, product_id: i32) -> Result<bool, Error> {
        sqlx::query!("DELETE FROM products WHERE id = $1", product_id)
            .execute(&self.0)
            .await?;

        Ok(true)
    }
}
