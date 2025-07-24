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
}

impl Error {
  pub fn invalid_url(url: &reqwest::Url) -> Self {
    Self::InvalidURL(url.to_string())
  }

  pub fn request_failed(err: reqwest::Error) -> Self {
    Self::RequestFailed(err)
  }

  pub fn write_failed(err: std::io::Error) -> Self {
    Self::WriteFailed(err)
  }
}

pub type Result<T> = std::result::Result<T, Error>;
