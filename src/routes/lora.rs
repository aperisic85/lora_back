use crate::{db, error::ApiError, models::{self, device::Device}};
use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::db::sensor::save_lora_packet;
use crate::models::device::CreateDevice;
use crate::models::sensor_data::{CreateLoraPacket, LoraPacket};
pub fn device_routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_devices))
        .route("/{id}", get(get_device_by_id))
        .route("/create", post(create_device))
}


pub fn lora_routes() -> Router<PgPool> {
    //Router::new().route("/lora-packets", post(handle_lora_packet))
    Router::new().route("/lora-packets", post(handle_lora_data))
}

async fn get_devices(State(pool): State<PgPool>) -> Result<Json<Vec<Device>>, ApiError> {
    let devices = db::sensor::get_all_devices(&pool).await?;
    Ok(Json(devices))
}

// 2. GET pojedinog uređaja po ID-u
async fn get_device_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Device>, ApiError> {
    let device = db::sensor::get_device_by_id(&pool, id).await?;
    Ok(Json(device))
}

// 3. POST za kreiranje novog uređaja
async fn create_device(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateDevice>,
) -> Result<Json<Device>, ApiError> {
    let device = db::sensor::create_device(&pool, payload).await?;
    Ok(Json(device))
}



// 4. POST za testiranje primljenih podataka sa senzora. Printa primljene podatke.
pub async fn test_sensor_data(
    Json(payload): Json<serde_json::Value>, // Koristimo serde_json::Value za fleksibilnost
) -> impl axum::response::IntoResponse {
    // Ispiši primljene podatke u log
    println!("Received sensor data: {:?}", payload);

    // Vrati podatke natrag kao odgovor
    Json(payload)
}

pub async fn handle_lora_packet(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<CreateLoraPacket>,
) -> Result<Json<LoraPacket>, ApiError> {
    let saved_packet = save_lora_packet(&pool, payload).await?;
    Ok(Json(saved_packet))
}


//#[axum::debug_handler]
pub async fn handle_lora_data(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<models::lora_data::SensorData>,
) -> Result<Json<models::lora_data::SensorData>, ApiError> {
    let data = db::lora::save_sensor_data(&pool, payload)
        .await
        .map_err(ApiError::DatabaseError)?;
    
    Ok(Json(data))
}