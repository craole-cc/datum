// -- Data Engineering Errors (types/data.rs) -- //

// region: Imports and Aliases
mod definition;
mod display;
mod implementation;
// endregion

// region: Dependencies
use std::{collections::HashMap, path::PathBuf, result};
// endregion

// region: Exports
pub use definition::*;
pub mod prelude {
  pub use super::{
    Category as DataErrorCategory, Error as DataError, Result as DataResult,
    Severity as DataErrorSeverity,
  };
}
// endregion
