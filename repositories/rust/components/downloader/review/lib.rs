//! # Downloader
//!
//! A concurrent file downloader that downloads multiple files from URLs to a local directory.
//!
//! ## Features
//!
//! - Concurrent downloads with optional concurrency limiting
//! - Atomic file operations (downloads to temp files, then atomically renames)
//! - Automatic filename extraction from URLs
//! - Comprehensive error handling and logging
//! - Automatic directory creation
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use downloader::Downloader;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let urls = vec![
//!     "https://example.com/file1.txt",
//!     "https://example.com/file2.pdf",
//! ];
//!
//! let downloader = Downloader::new(urls, "/download/path", Some(3));
//! downloader.start().await?;
//! # Ok(())
//! # }
//! ```

pub mod downloader;
pub mod error;
pub mod filename;
pub mod preview;
pub mod ui;

pub use downloader::Downloader;
pub use filename::FilenameExtractor;
pub use preview::{DownloadPreview, PreviewGenerator};
pub use ui::{ConsoleInterface, PreviewAction, UserInterface};

// Re-export common types
use crate::{Error, Result};
