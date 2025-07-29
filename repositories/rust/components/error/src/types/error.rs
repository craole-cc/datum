use super::*;

/// Main crate error enum integrating all error domains, including filesystem.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Data related errors.
  #[error(transparent)]
  Data(#[from] data::DataError),

  /// Filesystem related errors.
  #[error(transparent)]
  FileSystem(#[from] FileSystemError),
  // /// Validation/logic related errors.
  // #[error(transparent)]
  // Validation(#[from] validation::Error),
}

/// Result type alias for the error crate
pub type Result<T> = std::result::Result<T, Error>;

// region: Re-exports
pub use filesystem::prelude::*;
pub use severity::Severity;
//endregion
