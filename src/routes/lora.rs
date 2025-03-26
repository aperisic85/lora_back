use crate::{
    db,
    error::ApiError, models::device::Device,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use sqlx::PgPool;

use crate::models::device::{CreateDevice};
use crate::models::sensor_data::{SensorData, CreateSensorData};
use crate::db::sensor::create_sensor_data;

pub fn device_routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_devices))
        .route("/{id}", get(get_device_by_id))
        .route("/create", post(create_device))
}

// Ruta za senzorske podatke
pub fn sensor_data_routes() -> Router<PgPool> {
    Router::new()
        .route("/sensor-data", post(create_sensor_data_handler))
}


async fn get_devices(
    State(pool): State<PgPool>
) -> Result<Json<Vec<Device>>, ApiError> {
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

pub async fn create_sensor_data_handler(
    State(pool): State<PgPool>,
    Json(new_data): Json<CreateSensorData>,
) -> Result<Json<SensorData>, ApiError> {
    let sensor_data = create_sensor_data(&pool, new_data).await?;
    Ok(Json(sensor_data))
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