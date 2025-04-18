use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct Gateway {
    pub ant: i32,
    #[serde(rename = "gweui")]
    pub gw_eui: String,
    pub lat: f64,
    pub lon: f64,
    pub rssi: i32,
    pub snr: f64,
    pub time: String,  // Ili DateTime<Utc> ako želite parsirati
    pub ts: i64,
}
