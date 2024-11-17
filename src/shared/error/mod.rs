use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    SqlxError(#[from] sqlx::Error),
    Custom(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = match &self {
            Error::SqlxError(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "record not found!")
            }
            Error::SqlxError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
            Error::Custom(_) => (StatusCode::BAD_REQUEST, "bad request"),
        };

        let body = json!({
            "status": false,
            "message": status_code.1.to_string(),
        });

        (status_code.0, axum::Json(body)).into_response()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
