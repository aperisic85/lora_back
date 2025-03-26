use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local, Utc};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Device {
    pub id: Uuid,
    pub device_id: String,
    pub device_type: String,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDevice {
    pub device_id: String,
    pub device_type: String,
    pub description: Option<String>,
}

