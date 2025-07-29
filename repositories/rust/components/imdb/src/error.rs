use std::path::PathBuf;

/// Error types specific to IMDB dataset operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Invalid URL: {0}")]
  InvalidUrl(String),

  #[error("Invalid dataset name: '{0}'")]
  InvalidDatasetName(String),

  #[error("Invalid file path: {0}")]
  InvalidPath(PathBuf),

  #[error("Dataset configuration error: {0}")]
  ConfigError(String),

  #[error("Failed to create directory structure at: {0}")]
  PathCreation(PathBuf),

  #[error("Dataset not found: {0}")]
  DatasetNotFound(String),

  #[error("File operation failed: {0}")]
  FileOperation(String),

  #[error("IO error: {0}")]
  FileSystem(#[from] std::io::Error),

  #[error("URL parsing error: {0}")]
  UrlParse(#[from] url::ParseError),
}

/// Result type alias for this library
pub type Result<T> = std::result::Result<T, Error>;
