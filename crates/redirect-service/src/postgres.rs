use sqlx::{prelude::FromRow, PgPool};
use crate::Result;

#[derive(Debug, FromRow)]
pub struct Link {
    pub id: i64,
    pub short_id: String,
    pub original_url: String,
}

pub async fn build_dataase_pool() -> Result<PgPool> {
    Ok(PgPool::connect_lazy(
        "postgresql://postgres:postgres@localhost:5432/postgres",
    )
    .unwrap())
}
