use sqlx::{Error, PgPool};

use crate::api::users::models::user_model::UserModel;

use super::models::user_model::{UserModelRequest, UserModelResponse};

pub trait IUserRepository: Send + Sync {
    fn new(pool: PgPool) -> Self;

    fn create_user(
        &self,
        user: &UserModelRequest,
    ) -> impl std::future::Future<Output = Result<UserModelResponse, Error>> + Send;

    fn get_user_by_id(
        &self,
        id: i32,
    ) -> impl std::future::Future<Output = Result<UserModelResponse, Error>> + Send;

    fn get_user_by_email(
        &self,
        email: String,
    ) -> impl std::future::Future<Output = Result<UserModel, Error>> + Send;
}

#[derive(Debug, Clone)]
pub struct UserRepository(PgPool);

impl IUserRepository for UserRepository {
    fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    async fn create_user(&self, user: &UserModelRequest) -> Result<UserModelResponse, Error> {
        sqlx::query_as::<_, UserModelResponse>(
            "INSERT INTO users (name,email, password) VALUES ($1,$2,$3) RETURNING id,email,name,created_at",
        )
        .bind(user.name.clone())
        .bind(user.email.clone())
        .bind(user.password.clone())
        .fetch_one(&self.0)
        .await
    }

    async fn get_user_by_id(&self, id: i32) -> Result<UserModelResponse, Error> {
        let user = sqlx::query_as!(UserModel, "SELECT * FROM users where id = $1", id,)
            .fetch_one(&self.0)
            .await;

        match user {
            Ok(user) => Ok(UserModelResponse {
                id: user.id,
                email: user.email,
                name: user.name,
                created_at: user.created_at,
            }),
            Err(e) => Err(e),
        }
    }

    async fn get_user_by_email(&self, email: String) -> Result<UserModel, Error> {
        sqlx::query_as!(UserModel, "SELECT * FROM users where email = $1", email,)
            .fetch_one(&self.0)
            .await
    }
}
