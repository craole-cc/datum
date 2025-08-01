mod config;
mod mode;
mod via_deltalake;
mod via_polars;

// #[macro_use]
// extern crate anyhow;

use anyhow::{Context, anyhow, ensure};
use config::*;
use mode::ProcessingMode;
use std::{
  ffi::OsStr,
  fs::{File, metadata, read_dir, remove_file},
  io::Read,
  path::{Path, PathBuf},
  sync::Arc,
};
use tokio::task;
// use via_polars::execute;
use via_deltalake::execute;
// use polars::prelude::*;
type TheResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> TheResult<()> {
  // via_polars::execute().await?;
  execute().await?;
  Ok(())
}
