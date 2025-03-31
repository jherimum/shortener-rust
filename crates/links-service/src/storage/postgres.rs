use chrono::NaiveDateTime;
use sqlx::{query_as, PgPool};
use tap::TapFallible;
use super::{error::Error, model::ShortLink, Storage, StorageResult};

#[derive(Debug, Clone)]
pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    pub fn new() -> Self {
        todo!()
    }
}

#[async_trait::async_trait]
impl Storage for PostgresStorage {
    async fn create_link(
        &self,
        short_id: &str,
        original_url: &str,
        expires_at: Option<NaiveDateTime>,
    ) -> StorageResult<ShortLink> {
        query_as("INSERT INTO SHORT_LINK (short_id, original_url, expires_at)")
            .bind(short_id)
            .bind(original_url)
            .bind(expires_at)
            .fetch_one(&self.pool)
            .await
            .tap_err(|e| {
                log::error!("Failed to insert ShortLink on database: {e}")
            })
            .map_err(Error::from)
    }

    async fn get_link(
        &self,
        short_lonk_id: &str,
    ) -> StorageResult<Option<ShortLink>> {
        Ok(None)
    }
}
