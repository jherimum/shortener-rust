use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct ShortLink {
    pub id: i64,
    pub short_id: String,
    pub original_url: String,
    pub created_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
}
