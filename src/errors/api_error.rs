use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sqlx::error::Error as SqlxError;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError(SqlxError),
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::DatabaseError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
        };

        let body = json!({
            "error": message
        });

        (status, axum::Json(body)).into_response()
    }
}

impl From<SqlxError> for ApiError {
    fn from(err: SqlxError) -> Self {
        ApiError::DatabaseError(err)
    }
}
