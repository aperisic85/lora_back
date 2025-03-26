use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError (sqlx::Error),
    NotFound,
    InvalidHexData,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::DatabaseError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ),
            ApiError::InvalidHexData => (
                StatusCode::BAD_REQUEST,
                "Invalid hexadecimal data".to_string(),
            ),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                "Resource not found".to_string(),
            ),
        };
        
        // Return tuple with status code and JSON body
        (status, Json(json!({ "error": message }))).into_response()
    }
}