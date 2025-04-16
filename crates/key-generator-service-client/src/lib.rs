use proto::{KeyGenerateRequest, KeyGeneratorClient};
use tap::TapFallible;
use tonic::{transport::Channel, Request, Status};

pub type KgsClientResult<T> = Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    TonicTransportError(#[from] tonic::transport::Error),

    #[error("{0}")]
    StatusError(Status),
}

#[derive(Clone)]
pub struct KgsClient {
    client: KeyGeneratorClient<Channel>,
}

impl KgsClient {
    pub async fn new() -> KgsClientResult<Self> {
        let client = KeyGeneratorClient::connect("dst")
            .await
            .tap_err(|e| tracing::error!(" Failed to connect: {e}"))?;

        Ok(Self { client })
    }

    pub async fn generate(&mut self) -> KgsClientResult<String> {
        let request = Request::new(KeyGenerateRequest {});
        match self.client.generate(request).await {
            Ok(response) => Ok(response.into_inner().value),
            Err(status) => Err(Error::StatusError(status)),
        }
    }
}
