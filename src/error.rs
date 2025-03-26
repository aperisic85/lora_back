use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError(sqlx::Error),
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
        };

        let body = json!({
            "error": error_message,
        });

        (status, axum::Json(body)).into_response()
    }
}
