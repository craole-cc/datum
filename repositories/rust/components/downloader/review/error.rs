//! Error types and utilities for the downloader crate.

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Invalid URL: {0}")]
  InvalidURL(String),
  #[error("Invalid path: {0}")]
  InvalidPath(String),
  #[error("Request failed: {0}")]
  RequestFailed(#[from] reqwest::Error),
  #[error("Write failed: {0}")]
  WriteFailed(#[from] std::io::Error),
  #[error("Task failed: {0}")]
  TaskFailed(String),
  #[error("HTTP error {status}: {url}")]
  HttpError { status: u16, url: String },
}

impl Error {
  /// Creates an invalid URL error.
  pub fn invalid_url<S: AsRef<str>>(url: S) -> Self {
    Self::InvalidURL(url.as_ref().to_string())
  }

  /// Creates an invalid path error.
  pub fn invalid_path<S: AsRef<str>>(path: S) -> Self {
    Self::InvalidPath(path.as_ref().to_string())
  }

  /// Creates an HTTP error with status code and URL.
  pub fn http_error(status: u16, url: &str) -> Self {
    Self::HttpError {
      status,
      url: url.to_string(),
    }
  }

  /// Creates a task failed error.
  pub fn task_failed<S: AsRef<str>>(msg: S) -> Self {
    Self::TaskFailed(msg.as_ref().to_string())
  }
}

impl From<tokio::task::JoinError> for Error {
  fn from(err: tokio::task::JoinError) -> Self {
    Self::TaskFailed(err.to_string())
  }
}

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;
