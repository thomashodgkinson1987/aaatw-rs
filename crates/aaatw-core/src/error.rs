use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("image decode error: {0}")]
    Image(#[from] image::ImageError),
    #[error("invalid surface: expected {expected} bytes, got {got}")]
    InvalidSurface { expected: usize, got: usize },
    #[error("invalid rectangle: {0}")]
    InvalidRect(String),
    #[error("resource error: {0}")]
    Resource(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
