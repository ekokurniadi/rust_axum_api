use super::models::category_model::CategoryModel;
use sqlx::{Error, PgPool, Result};

pub trait ICategoryRepository: Send + Sync {
    fn new(pool: PgPool) -> Self;
    fn create_category(
        &self,
        name: String,
    ) -> impl std::future::Future<Output = Result<CategoryModel, Error>> + Send;
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
}
