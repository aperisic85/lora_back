use crate::error::ApiError;
use crate::models::device::{CreateDevice, Device};
use crate::models::sensor_data::{CreateLoraPacket, CreateSensorData, LoraPacket, SensorData};
use axum::{Json, extract::State};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Error, PgPool};

use tracing::info;

// Kreiranje novog uređaja
pub async fn create_device(pool: &PgPool, payload: CreateDevice) -> Result<Device, ApiError> {
    sqlx::query_as!(
        Device,
        r#"
        INSERT INTO devices (device_id, device_type, description)
        VALUES ($1, $2, $3)
        RETURNING
            id,
            device_id,
            device_type,
            description,
            created_at as "created_at: chrono::DateTime<Utc>"
        "#,
        payload.device_id,
        payload.device_type,
        payload.description
    )
    .fetch_one(pool)
    .await
    .map_err(ApiError::DatabaseError)
}

// Funkcija za dohvat podataka po device_id
// Dohvat uređaja po ID-u
pub async fn get_device_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Device, ApiError> {
    sqlx::query_as!(
        Device,
        r#"
        SELECT
            id,
            device_id,
            device_type,
            description,
            created_at as "created_at: chrono::DateTime<Utc>"
        FROM devices
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(ApiError::DatabaseError)?
    .ok_or(ApiError::NotFound)
}

pub async fn get_all_devices(pool: &PgPool) -> Result<Vec<Device>, ApiError> {
    sqlx::query_as!(
        Device,
        r#"
        SELECT
            id,
            device_id,
            device_type,
            description,
            created_at as "created_at: chrono::DateTime<Utc>"
        FROM devices
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(ApiError::DatabaseError)
}

// Glavna funkcija za unos podataka
pub async fn create_sensor_data(
    pool: &PgPool,
    new_data: CreateSensorData,
) -> Result<SensorData, ApiError> {
    sqlx::query_as!(
        SensorData,
        r#"
        INSERT INTO sensor_data (device_id, data, rssi, snr)
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            device_id,
            data as "data: _",
            received_at as "received_at: chrono::DateTime<Utc>",
            rssi,
            snr
        "#,
        new_data.device_id,
        new_data.data,
        new_data.rssi,
        new_data.snr
    )
    .fetch_one(pool)
    .await
    .map_err(ApiError::DatabaseError)
}

pub async fn save_lora_packet(
    pool: &PgPool,
    packet: CreateLoraPacket,
) -> Result<LoraPacket, Error> {
    let decoded_data = hex::decode(&packet.data).unwrap_or_default(); // Decode hex string to bytes

    sqlx::query_as!(
        LoraPacket,
        r#"
        INSERT INTO lora_packets (eui, devaddr, frequency, data, gateways)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id,
            eui,
            devaddr,
            frequency,
            data as "data: Vec<u8>",
            received_at as "received_at: chrono::DateTime<Utc>",
            gateways as "gateways: serde_json::Value"
        "#,
        packet.eui,
        packet.devaddr,
        packet.frequency,
        decoded_data,
        packet.gateways
    )
    .fetch_one(pool)
    .await
}
