use std::time::Duration;
use mobc::{Connection, Pool};
use mobc_redis::{
    redis::{AsyncCommands, Client},
    RedisConnectionManager,
};
use tap::TapFallible;
use tracing::instrument;
use crate::Result;

pub type RedisPool = Pool<RedisConnectionManager>;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

fn connect(connection_string: &str) -> Result<RedisPool> {
    let client = Client::open(connection_string)
        .tap_err(|e| log::error!("Failed to open redis client: {e}"))?;
    let manager = RedisConnectionManager::new(client);

    Ok(Pool::builder()
        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
        .max_open(CACHE_POOL_MAX_OPEN)
        .max_idle(CACHE_POOL_MAX_IDLE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .build(manager))
}

#[derive(Clone)]
pub struct Cache {
    pool: RedisPool,
}

impl Cache {
    pub fn from_connection_string(connection_string: &str) -> Result<Self> {
        let pool = connect(connection_string)?;
        Ok(Self::new(pool))
    }

    fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    async fn conn(&self) -> Result<Connection<RedisConnectionManager>> {
        Ok(self.pool.get().await?)
    }

    #[instrument(name = "get_link", skip(self))]
    pub async fn get_link(&self, id: &str) -> Result<Option<String>> {
        let mut conn = self.conn().await?;
        Ok(conn.get(id).await.unwrap())
    }

    #[instrument(name = "store_link", skip(self))]
    pub async fn store_link(
        &self,
        short_id: &str,
        original_url: &str,
        ttl: usize,
    ) -> Result<()> {
        let mut conn = self.conn().await?;
        let _: () = conn.set_ex(short_id, &original_url, ttl).await?;
        Ok(())
    }
}
