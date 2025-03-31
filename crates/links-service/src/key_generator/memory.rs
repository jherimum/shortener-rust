use std::collections::{HashMap, HashSet};

use super::KeyGenerator;

#[derive(Default)]
pub struct MemoryKeyGenerator {
    used: HashSet<String>,
}

impl MemoryKeyGenerator {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait::async_trait]
impl KeyGenerator for MemoryKeyGenerator {
    async fn create(&self) -> Option<String> {
        None
    }
}
