use sqlx::PgPool;

use super::{Error, KeyGenerator};

pub struct PostgresKeyGenerator {
    pool: PgPool,
}

impl PostgresKeyGenerator {
    pub fn from_connection_string(conn: &str) -> Self {
        todo!()
    }

    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl KeyGenerator for PostgresKeyGenerator {
    async fn generate(&self) -> Result<String, Error> {
        todo!()
    }
}
