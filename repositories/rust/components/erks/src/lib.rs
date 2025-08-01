// -- Erks Library Entrypoint (lib.rs) -- //

#![cfg_attr(feature = "unstable", feature(io_error_more))]
#![cfg_attr(feature = "unstable", feature(io_error_inprogress))]

// region: Imports
mod category;
mod context;
mod error;
mod implementation;
mod result;
mod severity;
// endregion

// region: Exports
pub mod prelude {
  // Internal-only prelude
  pub(crate) mod internal {
    pub use crate::*;
    pub(crate) use std::{
      any::Any,
      collections::HashMap,
      error::Error as StdError,
      fmt::{Display, Formatter, Result as FmtResult},
      io::{Error as IOError, ErrorKind},
      panic::Location as StdLocation,
      path::{Path, PathBuf},
      result::Result as StdResult,
    };
  }

  // Public-facing prelude
  pub use crate::{
    category::Category, context::Context, error::Error, result::Result,
    severity::Severity,
  };
}
pub use prelude::*;
pub use std::process::{ExitCode, ExitStatus, exit};
// Re-export essential miette types so users don't need to import miette directly
pub use miette::{
  Diagnostic, IntoDiagnostic, MietteHandlerOpts, Report,
  Severity as MietteSeverity, WrapErr, miette, set_hook, set_panic_hook,
};

// endregion
