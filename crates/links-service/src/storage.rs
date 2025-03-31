use chrono::NaiveDateTime;
use error::StorageResult;

mod error;
mod memory;
mod model;
mod postgres;

pub use model::ShortLink;
pub use error::Error;
pub use memory::MemoryStorage;

#[async_trait::async_trait]
pub trait Storage {
    async fn create_link(
        &self,
        short_id: &str,
        original_url: &str,
        expires_at: Option<NaiveDateTime>,
    ) -> StorageResult<ShortLink>;

    async fn get_link(
        &self,
        short_lonk_id: &str,
    ) -> StorageResult<Option<ShortLink>>;
}
