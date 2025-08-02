// -- Erks Library Entrypoint (lib.rs) -- //

// #![cfg_attr(feature = "unstable", feature(io_error_more))]
// #![cfg_attr(feature = "unstable", feature(io_error_inprogress))]

// region: Imports
// mod category;
// mod context;
// mod error;
// mod implementation;
// mod result;
// mod severity;
// endregion

// region: Exports
// pub mod prelude {
//   // Internal-only prelude
//   pub(crate) mod internal {
//     pub use crate::*;
//     pub(crate) use std::{
//       any::Any,
//       collections::{HashMap, HashSet},
//       error::Error as StdError,
//       ffi::OsStr,
//       fmt::{Display, Formatter, Result as FmtResult},
//       fs::{File, create_dir_all, metadata, read_dir, remove_file},
//       future::Future,
//       io::{BufRead, BufReader, Error as IOError, ErrorKind, Read},
//       panic::Location as StdLocation,
//       path::{Path, PathBuf},
//       process::{ExitCode, ExitStatus, exit},
//       result::Result as StdResult,
//       sync::{Arc, Mutex},
//       thread::sleep,
//       time::SystemTime,
//     };
//   }

//   // Public-facing prelude
//   // pub use crate::{
//   //   category::Category, context::Context, error::Error, result::Result,
//   //   severity::Severity,
//   // };
// }
// pub use prelude::*;

// TODO: Go back to making this work, for now re-export miette

// endregion

mod export;
pub use export::*;
