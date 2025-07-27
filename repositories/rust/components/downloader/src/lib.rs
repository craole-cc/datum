/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Internal modules
mod config;
mod core;
mod error;
mod events;
mod filename;
mod preview;
mod progress;
mod task;
mod utils;
mod validation;

// External dependencies that are part of the public API
pub use reqwest::{Client as ReqwestClient, Url as ReqwestUrl};
pub use tokio::sync::broadcast;

// Logging support
#[macro_use]
extern crate tracing;

// Re-export main types for convenience
pub use crate::{
  config::{Config, ConfigBuilder, OverwritePolicy},
  core::Downloader,
  error::{Error, ErrorKind, Result},
  events::{
    CollectingEventSink, CompositeEventSink, DownloadEvent, EventSink,
    LoggingEventSink
  },
  filename::{ConflictResolver, ConflictStrategy, Strategy},
  preview::{Conflict, Manifest, Status, Target},
  progress::{Reporter, Sender, Snapshot},
  task::{DownloadTask, TaskExecutor, TaskResult},
  utils::{download, download_with_config, format_filesize},
  validation::{Url, UrlValidator, UrlValidatorBuilder}
};
pub use std::{path::PathBuf, time::Duration};

mod prelude {
  pub use crate::{
    ConflictStrategy, DownloadEvent, Error, ErrorKind, EventSink,
    LoggingEventSink, OverwritePolicy, Result,
    config::Config,
    core::Downloader,
    filename::Strategy,
    preview::{Conflict, Manifest, Status, Target},
    progress::{Reporter, Sender, Snapshot},
    utils::{download, download_with_config},
    validation::UrlValidator
  };
}
