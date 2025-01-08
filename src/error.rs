use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Template error: {0}")]
    Template(#[from] tera::Error),
    #[error("Not found")]
    NotFound,
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Template(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        (status, message).into_response()
    }
}