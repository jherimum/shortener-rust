pub mod remote;

#[async_trait::async_trait]
pub trait KeyGenerator {
    async fn generate(&self) -> Option<String>;
}
