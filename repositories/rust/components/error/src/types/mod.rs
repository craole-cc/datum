// -- Errors (types/mod.rs) -- //

// region: Imports
mod data;
mod filesystem;
mod severity;
//endregion

// region: Exports
pub use data::prelude::*;
pub use filesystem::prelude::*;
pub use severity::Severity;
//endregion

use miette::Diagnostic;
use thiserror::Error;

/// Main crate error enum integrating all error domains.
#[derive(Debug, Error)]
pub enum Error {
  /// Data related errors.
  #[error(transparent)]
  Data(#[from] data::DataError),

  /// Filesystem related errors.
  #[error(transparent)]
  FileSystem(#[from] FileSystemError),
}

/// Result type alias for the error crate
pub type Result<T> = std::result::Result<T, Error>;
