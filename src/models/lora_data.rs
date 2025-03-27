use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::gateway::Gateway;
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct SensorData {
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
    pub received_at: Option<DateTime<Utc>>,
}