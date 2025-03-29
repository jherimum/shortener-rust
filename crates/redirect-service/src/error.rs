use std::io;
use mobc_redis::redis::RedisError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] io::Error),

    #[error("{0}")]
    RedisError(#[from] RedisError),

    #[error("{0}")]
    MobcRedisError(#[from] mobc::Error<RedisError>),

    #[error("{0}")]
    ClientError(#[from] links_service::ClientError),
}
