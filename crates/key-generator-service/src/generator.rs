use crate::key::Key;

mod memory;
mod postgres;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[async_trait::async_trait]
pub trait KeyGenerator {
    async fn generate(&self) -> Result<Key, Error>;
}
