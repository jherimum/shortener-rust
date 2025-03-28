use chrono::NaiveDateTime;
use sqlx::PgPool;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
}

pub struct Link {
    pub id: i64,
    pub short_id: String,
    pub original_url: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Storage {
    pool: PgPool,
}

impl Storage {
    pub async fn create_link(
        &self,
        original_url: &str,
    ) -> Result<Link, sqlx::Error> {
        todo!()
    }
}
