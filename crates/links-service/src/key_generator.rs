pub mod memory;
pub mod remote;

#[async_trait::async_trait]
pub trait KeyGenerator {
    async fn create(&self) -> Option<String>;
}
