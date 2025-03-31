pub type StorageResult<T> = Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Short link {0} already exists")]
    ShortLinkalteradyExists(String),
}
