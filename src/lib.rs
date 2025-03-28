pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod routes;

use axum::{Router, routing::get, routing::post};
use routes::lora::get_sensor_by_eui_handler;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
pub struct AppState {
    pool: sqlx::PgPool,
}


pub fn create_app(pool: PgPool) -> Router {
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    Router::new()
        .route("/health", get(routes::health::health_check))
        .route("/api/data/{eui}", axum::routing::get(get_sensor_by_eui_handler))
        //.route("/test", post(routes::lora::handle_lora_packet))
        .nest("/lora", routes::lora::device_routes())
        .nest("/api", routes::lora::lora_routes())
        
        .with_state(pool)
        .layer(cors)
}
