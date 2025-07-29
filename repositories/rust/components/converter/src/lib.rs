mod parquet;

// region: Macro Exports
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate anyhow;
// endregion

// region: Internal Exports
use anyhow::{Context, Result};
use polars::prelude::*;
use rayon::prelude::*;
use std::{
  fs::{File, metadata, read_dir, remove_file},
  io::{BufRead, BufReader},
  path::Path,
  sync::Mutex,
  time::SystemTime,
};
