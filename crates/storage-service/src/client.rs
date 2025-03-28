use reqwest::{Client as ReqwestClient, StatusCode, Url};
use serde::Deserialize;
use tap::TapFallible;
use url::ParseError;

pub type ClientResult<T> = Result<T, Error>;

#[derive(Debug, Deserialize)]
pub struct RestError {
    message: String,
}

#[derive(Debug, thiserror::Error)]
#[error("")]
pub enum ServerError {
    RestError(RestError),
    Unknown(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Parser error")]
    UtlParserError(#[from] ParseError),

    #[error("Parser error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Parser error")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parser error")]
    ClientError(#[from] ClientError),

    #[error("Parser error")]
    ServerError(#[from] ServerError),
}

#[derive(Debug, Deserialize)]
pub struct LinkModel {
    pub id: u64,
    pub short_id: String,
    pub original_url: String,
}

#[derive(Debug, Clone)]
pub struct Client {
    inner: ReqwestClient,
    base_url: Url,
}

impl Client {
    pub fn create(base_url: &str) -> ClientResult<Self> {
        Ok(Self::new(
            ReqwestClient::new(),
            Url::parse(base_url).unwrap(),
        ))
    }

    fn new(inner: ReqwestClient, base_url: Url) -> Self {
        Self { inner, base_url }
    }

    pub async fn find_link(
        &self,
        short_id: &str,
    ) -> ClientResult<Option<LinkModel>> {
        let url = self
            .base_url
            .join(&format!("links/{}", short_id))
            .map_err(ClientError::from)?;

        let response = self
            .inner
            .get(url)
            .send()
            .await
            .tap_err(|e| tracing::error!("Failed to fetch link: {e}"))
            .map_err(ClientError::from)?;

        match response.status() {
            StatusCode::OK => Ok(Some(
                response
                    .json()
                    .await
                    .tap_err(|e| {
                        tracing::error!("Failed to deserialize payload: {e}")
                    })
                    .map_err(ClientError::from)?,
            )),
            StatusCode::NOT_FOUND => Ok(None),
            _ => {
                let rest_error = response
                    .json::<RestError>()
                    .await
                    .map_err(ClientError::from)?;
                let rest_error = ServerError::RestError(rest_error);
                Err(rest_error.into())
            }
        }
    }
}
