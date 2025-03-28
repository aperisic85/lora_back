use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::error::ApiError;
use crate::models::lora_data::SensorData;
use crate::models::gateway::Gateway;
use sqlx::types::Json;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::http::StatusCode;


pub async fn save_sensor_data(
    pool: &sqlx::PgPool,
    data: SensorData,
) -> Result<SensorData, sqlx::Error> {
    sqlx::query_as!(
        SensorData,
        r#"
        INSERT INTO sensor_data (
            eui, ack, bat, cmd, confirmed, data, devaddr, dr, fcnt, freq,
            gws, offline, port, seqno, toa, ts
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        RETURNING
            id,
            eui,
            ack,
            bat,
            cmd,
            confirmed,
            data,
            devaddr,
            dr,
            fcnt,
            freq,
            gws as "gws: Json<Vec<Gateway>>",
            offline,
            port,
            seqno,
            toa,
            ts,
            received_at as "received_at: chrono::DateTime<Utc>"
        "#,
        data.eui,
        data.ack,
        data.bat,
        data.cmd,
        data.confirmed,
        data.data,
        data.devaddr,
        data.dr,
        data.fcnt,
        data.freq,
        serde_json::to_value(data.gws).unwrap(),
        data.offline,
        data.port,
        data.seqno,
        data.toa,
        data.ts
    )
    .fetch_one(pool)
    .await
}

use sqlx::PgPool;

pub async fn get_sensor_by_eui(
    pool: &PgPool,
    eui: String
) -> Result<SensorData, ApiError> {
    sqlx::query_as!(
        SensorData,
        r#"
        SELECT 
            id,
            eui,
            ack,
            bat,
            cmd,
            confirmed,
            data,
            devaddr,
            dr,
            fcnt,
            freq,
            gws as "gws: sqlx::types::Json<Vec<Gateway>>",
            offline,
            port,
            seqno,
            toa,
            ts,
            received_at as "received_at: chrono::DateTime<Utc>"
        FROM sensor_data
        WHERE eui = $1
        ORDER BY received_at DESC
        LIMIT 1
        "#,
        eui
    )
    .fetch_optional(pool)
    .await
    .map_err(ApiError::DatabaseError)?
    .ok_or(ApiError::NotFound)
}