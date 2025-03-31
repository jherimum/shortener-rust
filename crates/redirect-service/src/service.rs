use failsafe::futures::CircuitBreaker;
use tracing::instrument;

use crate::{cache::Cache, LinksServiceClient, Result};

#[derive(Clone)]
pub struct Service<CB> {
    pub cache: Cache,
    pub client: LinksServiceClient,
    pub circuit_breaker: CB,
}

impl<CB: CircuitBreaker> Service<CB> {
    pub fn new(
        cache: Cache,
        client: LinksServiceClient,
        circuit_breaker: CB,
    ) -> Self {
        Self {
            cache,
            client,
            circuit_breaker,
        }
    }

    #[instrument(name = "retrieve_link", skip(self))]
    pub async fn retrieve_link(&self, id: &str) -> Result<Option<String>> {
        if let Some(url) = self.cache.get_link(id).await? {
            return Ok(Some(url));
        }

        let link =
            match self.circuit_breaker.call(self.client.find_link(id)).await {
                Ok(link) => link,
                Err(e) => {
                    match e {
                        failsafe::Error::Inner(e) => {
                            log::error!("Failed: {e}")
                        }
                        failsafe::Error::Rejected => {
                            log::error!("Reject by circuit breaker")
                        }
                    };
                    None
                }
            };

        if let Some(link) = link {
            self.cache
                .store_link(&link.short_id, &link.original_url, 10)
                .await?;
            return Ok(Some(link.original_url.clone()));
        }

        Ok(None)
    }
}
