use std::time::Duration;
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, PgPool};
use crate::Result;

const POOL_MAX_CONN: u32 = 16;
const POOL_IDLE_TIMEOUT: u64 = 60;

#[derive(Debug, FromRow)]
pub struct Link {
    pub id: i64,
    pub short_id: String,
    pub original_url: String,
}

pub async fn build_dataase_pool() -> Result<PgPool> {
    PgPoolOptions::default()
        .max_connections(POOL_MAX_CONN)
        .idle_timeout(Some(Duration::from_secs(POOL_IDLE_TIMEOUT)));

    Ok(PgPool::connect_lazy(
        "postgresql://postgres:postgres@localhost:5432/postgres",
    )
    .unwrap())
}
