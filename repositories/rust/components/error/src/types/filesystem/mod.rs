// -- Filesystem-related Types (types/filesystem/mod.rs) -- //

// region: Imports and Aliases
mod definition;
mod display;
mod implementation;
// endregion

// region: Dependencies
use crate::Severity;
use definition::*;
use std::{
  collections::HashMap,
  fmt::{self, Display, Formatter},
  io,
  panic::Location,
  path::{Path, PathBuf},
  result,
};
// endregion

// region: Exports
pub mod prelude {
  pub use super::definition::{
    Category as FileSystemErrorCategory, Error as FileSystemError,
    Result as FileSystemResult,
  };
}
// endregion
