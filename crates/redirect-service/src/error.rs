use std::io;
use mobc_redis::redis::RedisError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),

    #[error("redis error: {0}")]
    RedisError(#[from] RedisError),

    #[error("redis error: {0}")]
    MobcRedis(#[from] mobc::Error<RedisError>),

    #[error("redis error: {0}")]
    ClientError(#[from] storage_service::ClientError),
}
