use reqwest::{Client as ReqwestClient, Url};

pub type ClientResult<T> = Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[derive(Clone)]
pub struct Client {
    inner: ReqwestClient,
    base_url: Url,
}

impl Client {
    pub fn new(base_url: Url) -> Self {
        Client {
            inner: ReqwestClient::new(),
            base_url,
        }
    }

    pub async fn get(&self, quantity: u8) -> ClientResult<Vec<String>> {
        Ok(vec![])
    }
}
