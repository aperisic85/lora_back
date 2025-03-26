use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SensorData {
    pub id: Uuid,
    pub device_id: String,
    pub data: serde_json::Value,
    pub received_at: DateTime<Utc>,
    pub rssi: Option<i32>,
    pub snr: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSensorData {
    pub device_id: String,
    pub data: serde_json::Value,
    pub rssi: Option<i32>,
    pub snr: Option<f64>,
}

