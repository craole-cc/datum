// -- Library (lib.rs) -- //

// region: Module Imports
// pub mod config;
// mod ingestion;
pub mod utils;
// endregion

// region: Macros
#[macro_use]
extern crate tracing;
// endregion

// region: Exports
// pub use config::Config;
// pub use errors::prelude::*;
pub use erks::{Context, Error, Result};
pub use imdb_dataset::*;
// pub use ingestion::*;
pub use polars::prelude::*;
pub use rayon::prelude::*;
pub use std::{
  fs::{File, metadata, read_dir, remove_file},
  io::{BufRead, BufReader},
  path::Path,
  sync::Mutex,
  time::SystemTime,
};
// pub use utils::*;
// endregion

// region: Public Exports
// pub use ingest::run;
// endregion

// -- End of the Library module (lib.rs) -- //
