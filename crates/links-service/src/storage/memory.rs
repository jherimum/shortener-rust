use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc},
};
use chrono::{NaiveDateTime, Utc};
use tokio::sync::Mutex;
use super::{error::StorageResult, ShortLink, Storage};

#[derive(Clone, Debug)]
pub struct MemoryStorage {
    data: Arc<Mutex<HashMap<String, ShortLink>>>,
    current_id: Arc<AtomicI64>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            current_id: Arc::new(AtomicI64::new(1)),
        }
    }
}

#[async_trait::async_trait]
impl Storage for MemoryStorage {
    async fn create_link(
        &self,
        short_link_id: &str,
        original_url: &str,
        expires_at: Option<NaiveDateTime>,
    ) -> StorageResult<ShortLink> {
        if self.data.lock().await.contains_key(short_link_id) {
            return Err(super::Error::ShortLinkalteradyExists(
                short_link_id.to_owned(),
            ));
        }

        let id = self
            .current_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let short_link = ShortLink {
            id,
            short_id: short_link_id.to_owned(),
            original_url: original_url.to_owned(),
            created_at: Utc::now().naive_local(),
            expires_at,
        };

        self.data
            .lock()
            .await
            .insert(short_link_id.to_owned(), short_link.clone());

        Ok(short_link)
    }

    async fn get_link(
        &self,
        short_lonk_id: &str,
    ) -> StorageResult<Option<ShortLink>> {
        Ok(self.data.lock().await.get(short_lonk_id).cloned())
    }
}
