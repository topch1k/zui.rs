#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    ZkError(#[from] zookeeper_async::ZkError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Establish connection timeout")]
    ConnectionTimeoutError,
}

pub type AppResult<T> = Result<T, AppError>;
