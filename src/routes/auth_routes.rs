use crate::{
    api::auth::handlers::{login_user, register_user},
    state::AppState,
};
use axum::{routing::post, Router};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login_user::execute))
        .route("/auth/register", post(register_user::execute))
}
