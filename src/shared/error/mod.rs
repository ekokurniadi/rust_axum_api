use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::error::Error as SError;
use serde_json::json;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    SqlxError(#[from] sqlx::Error),
    Custom(String),
    InvalidToken,
    ExpiredToken,
    WrongCredentials,
    TokenCreation,
    MissingCredentials,
    BcryptError(#[from] bcrypt::BcryptError),
    RecordAlreadyExists,
    DeserializationError(#[from] SError), // Handling deserialization errors
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = match &self {
            Error::SqlxError(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "record not found!")
            }
            Error::SqlxError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
            Error::Custom(e) => (StatusCode::BAD_REQUEST, e.as_ref()),
            Error::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "token creation error"),
            Error::WrongCredentials => (StatusCode::UNAUTHORIZED, "wrong credentials"),
            Error::MissingCredentials => (StatusCode::UNAUTHORIZED, "missing credentials"),
            Error::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid token"),
            Error::ExpiredToken => (StatusCode::UNAUTHORIZED, "expired token"),
            Error::BcryptError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "failed hash password"),
            Error::RecordAlreadyExists => (StatusCode::CONFLICT, "record already exists"),
            Error::DeserializationError(_) => (StatusCode::BAD_REQUEST, "bad request"),
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
