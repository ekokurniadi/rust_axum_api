use axum::{routing::post, Router};

use crate::{api::users::handlers::create_user, state::AppState};

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/", post(create_user::execute))
}
