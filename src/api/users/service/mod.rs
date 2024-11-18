use bcrypt::DEFAULT_COST;

use crate::{
    api::auth::models::{LoginRequest, LoginResponse},
    shared::error::Error,
    utils::jwt::generate_token,
};

use super::{
    models::user_model::{UserModelRequest, UserModelResponse},
    repository::{IUserRepository, UserRepository},
};
use std::sync::Arc;

pub trait IUserService: Send + Sync {
    fn new(user_repo: Arc<UserRepository>) -> Self;

    fn create_user(
        &self,
        user: &UserModelRequest,
    ) -> impl std::future::Future<Output = Result<UserModelResponse, Error>> + Send;

    fn get_user_by_id(
        &self,
        id: i32,
    ) -> impl std::future::Future<Output = Result<UserModelResponse, Error>> + Send;

    fn login(
        &self,
        login_request: &LoginRequest,
    ) -> impl std::future::Future<Output = Result<LoginResponse, Error>> + Send;
}

#[derive(Clone, Debug)]
pub struct UserService {
    user_repo: Arc<UserRepository>,
}

impl IUserService for UserService {
    fn new(user_repo: Arc<UserRepository>) -> Self {
        Self { user_repo }
    }

    async fn create_user(&self, user: &UserModelRequest) -> Result<UserModelResponse, Error> {
        let existing = self.user_repo.get_user_by_email(user.email.clone()).await;

        let exists = existing.is_ok();

        if !exists {
            let hash_password = bcrypt::hash(user.password.clone(), DEFAULT_COST)?;

            let mut user_with_hashed_password = user.clone();
            user_with_hashed_password.password = hash_password;

            let create_user = self.user_repo.create_user(&user_with_hashed_password).await;

            match create_user {
                Err(e) => Err(Error::SqlxError(e)),
                Ok(res) => Ok(res),
            }
        } else {
            Err(Error::RecordAlreadyExists)
        }
    }

    async fn get_user_by_id(&self, id: i32) -> Result<UserModelResponse, Error> {
        let user = self.user_repo.get_user_by_id(id).await;

        match user {
            Err(e) => Err(Error::SqlxError(e)),
            Ok(res) => Ok(res),
        }
    }

    async fn login(&self, login_request: &LoginRequest) -> Result<LoginResponse, Error> {
        let user = self
            .user_repo
            .get_user_by_email(login_request.email.clone())
            .await
            .map_err(|_| Error::WrongCredentials)?;

        // Verify the password
        let is_password_valid = bcrypt::verify(&login_request.password, &user.password)
            .map_err(|_| Error::WrongCredentials)?;

        if !is_password_valid {
            return Err(Error::WrongCredentials);
        }

        let token = generate_token(user.id);

        Ok(LoginResponse {
            user: UserModelResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                created_at: user.created_at,
            },
            token,
        })
    }
}
