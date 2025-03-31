use std::{collections::VecDeque, sync::Arc};
use failsafe::futures::CircuitBreaker;
use key_generator_service_client::Client as KeyGeneratorServiceClient;
use tokio::sync::Mutex;
use super::KeyGenerator;

type Queue = Arc<Mutex<VecDeque<String>>>;

#[derive(Clone)]
pub struct RemoteKeyGenerator<CB> {
    client: KeyGeneratorServiceClient,
    queue: Queue,
    circuit_breaker: CB,
}

impl<CB> RemoteKeyGenerator<CB>
where
    CB: CircuitBreaker,
{
    pub fn new(client: KeyGeneratorServiceClient, circuit_breaker: CB) -> Self {
        Self {
            client,
            queue: Arc::new(Mutex::new(VecDeque::with_capacity(50))),
            circuit_breaker,
        }
    }

    async fn retrieve_ids(&self, quantity: u8) -> Vec<String> {
        match self.circuit_breaker.call(self.client.get(quantity)).await {
            Ok(keys) => keys,
            Err(failsafe::Error::Inner(e)) => vec![],
            Err(failsafe::Error::Rejected) => vec![],
        }
    }
}

#[async_trait::async_trait]
impl<CB: CircuitBreaker + Send + Sync> KeyGenerator for RemoteKeyGenerator<CB> {
    async fn create(&self) -> Option<String> {
        let mut queue = self.queue.lock().await;
        if queue.is_empty() {
            let keys = self.retrieve_ids(50).await;
            keys.into_iter().for_each(|key| queue.push_front(key));
        }
        queue.pop_front()
    }
}
