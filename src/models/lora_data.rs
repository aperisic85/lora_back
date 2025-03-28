use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::gateway::Gateway;
use sqlx::FromRow;
use sqlx::types::Json;

#[derive(Debug, Deserialize, Serialize, FromRow)]

pub struct SensorData {
    pub id:i32,  // 👈 Must be included in the query
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
    #[serde(rename = "gws")]
    #[sqlx(rename = "gws")] 
    pub gws: Json<Vec<Gateway>>,
    pub offline: bool,
    pub port: i32,
    pub seqno: i32,
    pub toa: i32,
    pub ts: i64,
    pub received_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSensorData {
    #[serde(flatten)]
    pub data: SensorData,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BasicSensorData {
    #[serde(rename = "EUI")]
    pub eui: String,
    pub data: String,
    pub ts: i64,
    pub received_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FrontLoraData {
    pub id: i32,  // 👈 Add this field
    #[serde(rename = "EUI")]
    pub eui: String,
    pub data: String,
    pub ts: i64,
    pub received_at: chrono::DateTime<Utc>,
}
