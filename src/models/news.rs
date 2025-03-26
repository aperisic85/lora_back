use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewsPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>, // Bez Option<>
}
#[derive(Debug, Deserialize)]
pub struct CreateNewsPost {
    pub title: String,
    pub content: String,
}
