use std::{collections::HashMap, sync::atomic::AtomicU64};
use tokio::sync::Mutex;

use crate::key::Key;
use super::{Error, KeyGenerator};

#[derive(Debug)]
pub struct InMemoryKeyGenerator {
    counter: AtomicU64,
    key_size: u8,
    available_keys: Mutex<HashMap<u64, Key>>,
    used_keys: Mutex<HashMap<u64, Key>>,
    fetch_size: u64,
}

impl InMemoryKeyGenerator {
    pub fn new(key_size: u8, fetch_size: u64) -> Self {
        Self {
            counter: AtomicU64::new(1),
            key_size,
            available_keys: Default::default(),
            used_keys: Default::default(),
            fetch_size,
        }
    }
}

#[async_trait::async_trait]
impl KeyGenerator for InMemoryKeyGenerator {
    async fn generate(&self, quantity: u64) -> Result<Vec<Key>, Error> {
        let quantity = quantity as usize;
        {
            let mut available_lock = self.available_keys.lock().await;

            while available_lock.len() < quantity {
                let old = self
                    .counter
                    .fetch_add(100, std::sync::atomic::Ordering::SeqCst);

                let generated = Key::generate_multiple(
                    old..old + self.fetch_size,
                    self.key_size,
                );

                available_lock.extend(generated);
            }
        }

        let mut available_lock = self.available_keys.lock().await;
        let removed_keys = available_lock
            .iter()
            .take(quantity)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();

        for k in &removed_keys {
            available_lock.remove(&k.0);
        }

        let mut used_lock = self.used_keys.lock().await;
        used_lock.extend(removed_keys.clone());

        Ok(removed_keys.into_iter().map(|key| key.1).collect())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn name() {
        let mut generator = InMemoryKeyGenerator::new(7, 100);
        assert_eq!(
            1,
            generator.counter.load(std::sync::atomic::Ordering::Relaxed),
        );
        assert_eq!(&HashMap::new(), generator.available_keys.get_mut());
        assert_eq!(&HashMap::new(), generator.used_keys.get_mut());

        let keys = generator.generate(10).await.unwrap();
        assert_eq!(10, keys.len());

        dbg!(generator);
    }
}
