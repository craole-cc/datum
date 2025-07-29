// -- Data Engineering Errors (types/data.rs) -- //

// region: Imports and Aliases
mod definition;
mod display;
mod implementation;
pub mod prelude;
// endregion

// region: Dependencies
use crate::Severity;
use definition::*;
use std::{collections::HashMap, path::PathBuf, result};
// endregion

// region: Exports
pub use prelude::*;
// endregion
