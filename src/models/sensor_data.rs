use super::gateway::Gateway;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorData2 {
    #[serde(rename = "EUI")]
    pub eui: String,
    pub ack: bool,
    pub bat: i32,
    pub cmd: String,
    pub confirmed: bool,
    pub data: String,
    pub devaddr: String,
    pub dr: String,
    pub fcnt: i32,
    pub freq: i64,
    pub gws: Vec<Gateway>,
    pub offline: bool,
    pub port: i32,
    pub seqno: i32,
    pub toa: i32,
    pub ts: i64,
}

// For database responses
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct LoraPacket {
    pub id: i32,
    pub eui: String,
    pub devaddr: String,
    pub frequency: i64,
    pub data: String,
    pub received_at: DateTime<Utc>,
    pub gateways: serde_json::Value,
}
#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct CreateLoraPacket {
    pub eui: String,
    pub devaddr: String,
    pub frequency: i64,
    pub data: String,                // BYTEA
    pub gateways: serde_json::Value, // dynamic gateway data
}
