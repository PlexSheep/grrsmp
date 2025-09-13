use thiserror::Error;
use tokio_rustls::rustls;

pub type CoreResult<T> = std::result::Result<T, CoreError>;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("standard io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("TLS error: {0}")]
    TLS(#[from] rustls::Error),
}
