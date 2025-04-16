use failsafe::futures::CircuitBreaker;
use key_generator_service_client::KgsClient;
use super::KeyGenerator;

#[derive(Clone)]
pub struct RemoteKeyGenerator<CB> {
    client: KgsClient,
    circuit_breaker: CB,
}

impl<CB> RemoteKeyGenerator<CB>
where
    CB: CircuitBreaker,
{
    pub fn new(client: KgsClient, circuit_breaker: CB) -> Self {
        Self {
            client,
            circuit_breaker,
        }
    }
}

#[async_trait::async_trait]
impl<CB: CircuitBreaker + Send + Sync> KeyGenerator for RemoteKeyGenerator<CB> {
    async fn generate(&self) -> Option<String> {
        let mut client = self.client.clone();
        match client.generate().await {
            Ok(key) => Some(key),
            Err(_) => todo!(),
        }
    }
}
