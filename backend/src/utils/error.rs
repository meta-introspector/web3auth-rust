use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ApiError {
    #[error("Kesalahan autentikasi: {0}")]
    AuthError(String),

    #[error("Resource tidak ditemukan: {0}")]
    NotFoundError(String),

    #[error("Data input tidak valid: {0}")]
    ValidationError(String),

    #[error("Error internal server: {0}")]
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::AuthError(message) => (StatusCode::UNAUTHORIZED, message),
            Self::NotFoundError(message) => (StatusCode::NOT_FOUND, message),
            Self::ValidationError(message) => (StatusCode::BAD_REQUEST, message),
            Self::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::InternalServerError(format!("Error internal server: {}", err))
    }
} 