use std::sync::atomic::{AtomicU64, Ordering};
use crate::key::Key;
use super::{Error, KeyGenerator};

#[derive(Debug)]
pub struct InMemoryKeyGenerator {
    counter: AtomicU64,
    key_size: u8,
}

impl InMemoryKeyGenerator {
    pub fn new(key_size: u8) -> Self {
        Self {
            counter: AtomicU64::new(1),
            key_size,
        }
    }
}

#[async_trait::async_trait]
impl KeyGenerator for InMemoryKeyGenerator {
    async fn generate(&self) -> Result<Key, Error> {
        let number = self.counter.fetch_add(1, Ordering::SeqCst);
        Ok(Key::generate(number, self.key_size))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn name() {}
}
