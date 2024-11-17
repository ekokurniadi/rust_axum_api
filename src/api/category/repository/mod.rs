use super::models::{category_model::CategoryModel, category_model_request::CategoryModelRequest};
use sqlx::{Error, PgPool, Result};

pub trait ICategoryRepository: Send + Sync {
    fn new(pool: PgPool) -> Self;

    fn create_category(
        &self,
        name: String,
    ) -> impl std::future::Future<Output = Result<CategoryModel, Error>> + Send;

    fn get_categories(
        &self,
        page: i64,
        limit: i64,
    ) -> impl std::future::Future<Output = (Result<Vec<CategoryModel>, Error>, i64)> + Send;

    fn get_category_by_id(
        &self,
        id: i32,
    ) -> impl std::future::Future<Output = Result<CategoryModel, Error>> + Send;

    fn update_category(
        &self,
        category_id: i32,
        category: &CategoryModelRequest,
    ) -> impl std::future::Future<Output = Result<CategoryModel, Error>> + Send;

    fn delete_category(
        &self,
        category_id: i32,
    ) -> impl std::future::Future<Output = Result<bool, Error>> + Send;
}

#[derive(Debug, Clone)]
pub struct CategoryRepository(PgPool);

impl ICategoryRepository for CategoryRepository {
    fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    async fn create_category(&self, name: String) -> Result<CategoryModel, Error> {
        sqlx::query_as::<_, CategoryModel>("INSERT INTO category (name) VALUES ($1) RETURNING *")
            .bind(name)
            .fetch_one(&self.0)
            .await
    }

    async fn get_categories(
        &self,
        page: i64,
        limit: i64,
    ) -> (Result<Vec<CategoryModel>, Error>, i64) {
        let offset = (page - 1) * limit;
        let result = sqlx::query_as!(
            CategoryModel,
            "SELECT * FROM category order by id LIMIT $1 OFFSET $2",
            limit,
            offset,
        )
        .fetch_all(&self.0)
        .await;

        let total_count = sqlx::query_scalar!("SELECT COUNT(*) FROM category")
            .fetch_one(&self.0)
            .await
            .unwrap_or(Some(0));

        (result, total_count.unwrap_or(0))
    }

    async fn get_category_by_id(&self, id: i32) -> Result<CategoryModel, Error> {
        sqlx::query_as!(CategoryModel, "SELECT * FROM category where id = $1", id,)
            .fetch_one(&self.0)
            .await
    }

    async fn update_category(
        &self,
        category_id: i32,
        category: &CategoryModelRequest,
    ) -> Result<CategoryModel, Error> {
        sqlx::query_as::<_, CategoryModel>(
            "UPDATE category set name = $1 where id = $2 RETURNING *",
        )
        .bind(category.name.clone())
        .bind(category_id)
        .fetch_one(&self.0)
        .await
    }

    async fn delete_category(&self, category_id: i32) -> Result<bool, Error> {
        sqlx::query!("DELETE FROM category WHERE id = $1", category_id)
            .execute(&self.0)
            .await?;

        Ok(true)
    }
}
