use sqlx::PgPool;

use crate::key::Key;

use super::{Error, KeyGenerator};

pub struct PostgresKeyGenerator;

#[async_trait::async_trait]
impl KeyGenerator for PostgresKeyGenerator {
    async fn generate(&self) -> Result<Key, Error> {
        todo!()
    }
}
