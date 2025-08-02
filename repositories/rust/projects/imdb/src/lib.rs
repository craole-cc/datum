// -- Library (projects/imdb/src/lib.rs) -- //

// -- Module Imports
pub mod config;

// -- Internal Imports
pub(crate) use erks::*; // Errors and Tracing
pub(crate) use utils::*; // Reuable functions

// -- External Imports
pub(crate) use futures::future::join_all;
pub(crate) use std::{
  collections::{HashMap, HashSet},
  ffi::OsStr,
  fs::{File, create_dir_all, metadata, read_dir, remove_file},
  future::Future,
  io::{BufRead, BufReader, Read},
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
  thread::sleep,
  time::SystemTime,
};

// -- End of the Library module (lib.rs) -- //
