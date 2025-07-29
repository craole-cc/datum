mod core;
mod dataset;
mod error;
mod info;
mod path;

pub use core::Datasets;
pub use dataset::Dataset;
pub use error::{Error, Result};
pub use info::*;
pub use path::{Files, Home, Paths};
use std::fs::metadata;

// Re-export commonly used types for convenience
// pub mod prelude {
//   pub use crate::{Dataset, Datasets, Error, Files, Home, Result};
// }
