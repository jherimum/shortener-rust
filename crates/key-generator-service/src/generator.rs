use crate::key::Key;

mod memory;
mod postgres;

pub use memory::InMemoryKeyGenerator;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[async_trait::async_trait]
pub trait KeyGenerator {
    async fn generate(&self, quantity: u64) -> Result<Vec<Key>, Error>;
}
