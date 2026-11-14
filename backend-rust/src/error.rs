use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de comunicación con CouchDB: {0}")]
    DatabaseError(#[from] reqwest::Error),

    #[error("Error de validación de negocio: {0}")]
    ValidationError(String),

    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Error en validación de DTE: {0}")]
    DteError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::DatabaseError(e) => (
                StatusCode::BAD_GATEWAY,
                format!("Fallo de persistencia local: {}", e),
            ),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::DteError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
        };

        let body = Json(json!({
            "status": "error",
            "message": error_message
        }));

        (status, body).into_response()
    }
}