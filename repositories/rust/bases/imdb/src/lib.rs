// -- Library (lib.rs) -- //

// region: Module Imports
mod cli;
mod commands;
mod error;
mod utils;
// endregion

// region: Macros
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate anyhow;
// endregion

// region: Exports
use clap::{Parser, Subcommand};
use cli::Cli;
// use commands;
use error::{Context, Result};
use utils::*;
// endregion

// region: Public Exports
pub use cli::run;
// endregion

// -- End of the Library module (lib.rs) -- //
