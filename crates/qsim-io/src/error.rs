//! I/O error types

use thiserror::Error;

/// I/O error types
#[derive(Error, Debug)]
pub enum IoError {
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid data: {0}")]
    InvalidData(String),
}
