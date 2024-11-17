use sqlx::Error;

use super::{
    models::{category_model::CategoryModel, category_model_request::CategoryModelRequest},
    repository::{CategoryRepository, ICategoryRepository},
};
use std::sync::Arc;

pub trait ICategoryService: Send + Sync {
    fn new(category_repository: Arc<CategoryRepository>) -> Self;

    fn create_new(
        &self,
        category: &CategoryModelRequest,
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
#[derive(Clone, Debug)]
pub struct CategoryService {
    category_repository: Arc<CategoryRepository>,
}

impl ICategoryService for CategoryService {
    fn new(category_repository: Arc<CategoryRepository>) -> Self {
        Self {
            category_repository,
        }
    }

    async fn create_new(&self, category: &CategoryModelRequest) -> Result<CategoryModel, Error> {
        self.category_repository
            .create_category(category.name.clone())
            .await
    }

    async fn get_categories(
        &self,
        page: i64,
        limit: i64,
    ) -> (Result<Vec<CategoryModel>, Error>, i64) {
        self.category_repository.get_categories(page, limit).await
    }

    async fn get_category_by_id(&self, id: i32) -> Result<CategoryModel, Error> {
        self.category_repository.get_category_by_id(id).await
    }

    async fn update_category(
        &self,
        category_id: i32,
        category: &CategoryModelRequest,
    ) -> Result<CategoryModel, Error> {
        self.category_repository
            .update_category(category_id, category)
            .await
    }

    async fn delete_category(&self, category_id: i32) -> Result<bool, Error> {
        self.category_repository.delete_category(category_id).await
    }
}
