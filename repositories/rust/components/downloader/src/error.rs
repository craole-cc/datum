//! Enhanced error types with comprehensive error information
//!
//! This module provides detailed error types that give users actionable
//! information about what went wrong and how to potentially fix it.

use std::{collections::HashMap, path::PathBuf};

/// Comprehensive error type for the downloader crate.
///
/// This enum provides detailed error information to help users
/// understand and resolve issues during downloads.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Invalid URL format or unsupported protocol
  #[error("Invalid URL '{url}': {reason}")]
  InvalidUrl { url: String, reason: String },

  /// Invalid file path or permission issues
  #[error("Invalid path '{path}': {reason}")]
  InvalidPath { path: String, reason: String },

  /// HTTP request failed (network issues, DNS, etc.)
  #[error("Request failed for '{url}': {download}")]
  RequestFailed {
    url: String,
    #[source]
    download: reqwest::Error,
  },

  /// HTTP server returned an error status
  #[error("HTTP {status} error for '{url}': {message}")]
  HttpStatus {
    status: u16,
    url: String,
    message: String,
  },

  /// File system operation failed
  #[error("I/O operation failed: {message}")]
  FileSystem { message: String },

  /// Download task failed or panicked
  #[error("Task {index} failed: {reason}")]
  TaskFailed { index: usize, reason: String },

  /// File already exists and overwrite policy prevents overwrite
  #[error("File already exists: {}", .0.display())]
  FileExists(PathBuf),

  /// Multiple files already exist
  #[error("Multiple files already exist: {}", format_existing_files(.files))]
  ExistingFiles { files: HashMap<String, PathBuf> },

  /// File size exceeds configured limits
  #[error("File too large: {size} bytes (limit: {max_size} bytes)")]
  FileTooLarge { size: u64, max_size: u64 },

  /// Filename extraction or validation failed
  #[error("Filename error: {message}")]
  Filename { message: String },

  /// URL validation failed
  #[error("URL validation failed: {message}")]
  Validation { message: String },

  /// Configuration error
  #[error("Configuration error: {message}")]
  Configuration { message: String },

  /// Authentication or authorization failed
  #[error("Authentication failed for '{url}': {message}")]
  Authentication { url: String, message: String },

  /// SSL/TLS certificate error
  #[error("Certificate error for '{url}': {message}")]
  Certificate { url: String, message: String },

  /// Timeout occurred during download
  #[error("Timeout after {duration}s for '{url}'")]
  Timeout { url: String, duration: u64 },

  /// Insufficient disk space
  #[error(
    "Insufficient disk space: need {needed} bytes, have {available} bytes"
  )]
  InsufficientSpace { needed: u64, available: u64 },

  /// Content validation failed (checksums, etc.)
  #[error("Content validation failed for '{url}': {reason}")]
  ContentValidation { url: String, reason: String },

  /// Too many redirects
  #[error("Too many redirects for '{url}': {count} (limit: {limit})")]
  TooManyRedirects {
    url: String,
    count: usize,
    limit: usize,
  },

  #[error("No filename could be extracted from URL: {0}")]
  MissingFilename(String),

  #[error("Invalid characters in filename: {0}")]
  InvalidCharacters(String),

  #[error("Filename too long: {0} (max: {1})")]
  FilenameTooLong(usize, usize),

  #[error("Reserved filename: {0}")]
  ReservedFilename(String),

  /// Unsupported content type or format
  #[error("Unsupported content type '{content_type}' for '{url}'")]
  UnsupportedContentType { url: String, content_type: String },

  /// Rate limiting or server restrictions
  #[error("Rate limited by server for '{url}': retry after {retry_after}s")]
  RateLimited { url: String, retry_after: u64 },

  /// Multiple errors occurred (batch operations)
  #[error("Multiple errors occurred: {}", format_multiple_errors(.errors))]
  MultipleErrors { errors: Vec<Error> },
}

/// Convenience type alias for Results.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
  /// Creates an invalid URL error with context.
  pub fn invalid_url<S: AsRef<str>>(url: S, reason: S) -> Self {
    Self::InvalidUrl {
      url: url.as_ref().to_string(),
      reason: reason.as_ref().to_string(),
    }
  }

  /// Creates an invalid path error with context.
  pub fn invalid_path<S: AsRef<str>>(path: S, reason: S) -> Self {
    Self::InvalidPath {
      path: path.as_ref().to_string(),
      reason: reason.as_ref().to_string(),
    }
  }

  /// Creates an HTTP error with detailed information.
  pub fn http_error<S: AsRef<str>>(status: u16, url: S, message: S) -> Self {
    Self::HttpStatus {
      status,
      url: url.as_ref().to_string(),
      message: message.as_ref().to_string(),
    }
  }

  /// Creates an I/O error with context.
  pub fn io_error<S: AsRef<str>>(message: S) -> Self {
    Self::FileSystem {
      message: message.as_ref().to_string(),
    }
  }

  /// Creates a task failure error.
  pub fn task_failed(index: usize, reason: String) -> Self {
    Self::TaskFailed { index, reason }
  }

  /// Creates an existing files error.
  pub fn existing_files(files: HashMap<String, PathBuf>) -> Self {
    Self::ExistingFiles { files }
  }

  /// Creates a file too large error.
  pub fn file_too_large(size: u64, max_size: u64) -> Self {
    Self::FileTooLarge { size, max_size }
  }

  /// Creates a filename error.
  pub fn filename_error<S: AsRef<str>>(message: S) -> Self {
    Self::Filename {
      message: message.as_ref().to_string(),
    }
  }

  /// Creates a validation error.
  pub fn validation_error<S: AsRef<str>>(message: S) -> Self {
    Self::Validation {
      message: message.as_ref().to_string(),
    }
  }

  /// Creates an authentication error.
  pub fn authentication_error<S: AsRef<str>>(url: S, message: S) -> Self {
    Self::Authentication {
      url: url.as_ref().to_string(),
      message: message.as_ref().to_string(),
    }
  }

  /// Creates a certificate error.
  pub fn certificate_error<S: AsRef<str>>(url: S, message: S) -> Self {
    Self::Certificate {
      url: url.as_ref().to_string(),
      message: message.as_ref().to_string(),
    }
  }

  /// Creates a timeout error.
  pub fn timeout_error<S: AsRef<str>>(url: S, duration: u64) -> Self {
    Self::Timeout {
      url: url.as_ref().to_string(),
      duration,
    }
  }

  /// Creates an insufficient space error.
  pub fn insufficient_space(needed: u64, available: u64) -> Self {
    Self::InsufficientSpace { needed, available }
  }

  /// Creates a content validation error.
  pub fn content_validation_error<S: AsRef<str>>(url: S, reason: S) -> Self {
    Self::ContentValidation {
      url: url.as_ref().to_string(),
      reason: reason.as_ref().to_string(),
    }
  }

  /// Creates a too many redirects error.
  pub fn too_many_redirects<S: AsRef<str>>(
    url: S,
    count: usize,
    limit: usize,
  ) -> Self {
    Self::TooManyRedirects {
      url: url.as_ref().to_string(),
      count,
      limit,
    }
  }

  /// Creates an unsupported content type error.
  pub fn unsupported_content_type<S: AsRef<str>>(
    url: S,
    content_type: S,
  ) -> Self {
    Self::UnsupportedContentType {
      url: url.as_ref().to_string(),
      content_type: content_type.as_ref().to_string(),
    }
  }

  /// Creates a rate limited error.
  pub fn rate_limited<S: AsRef<str>>(url: S, retry_after: u64) -> Self {
    Self::RateLimited {
      url: url.as_ref().to_string(),
      retry_after,
    }
  }

  /// Creates a multiple errors error.
  pub fn multiple_errors(errors: Vec<Error>) -> Self {
    Self::MultipleErrors { errors }
  }

  /// Returns true if this error is recoverable (should retry).
  pub fn is_recoverable(&self) -> bool {
    match self {
      // Network issues that might be temporary
      Error::RequestFailed { .. } => true,
      Error::Timeout { .. } => true,
      Error::RateLimited { .. } => true,

      // HTTP errors that might be temporary
      Error::HttpStatus { status, .. } => {
        matches!(*status, 429 | 500 | 502 | 503 | 504)
      }

      // I/O errors that might be temporary
      Error::FileSystem { message } => {
        message.contains("temporarily unavailable")
          || message.contains("timeout")
          || message.contains("connection")
      }

      // All other errors are generally not recoverable
      _ => false,
    }
  }

  /// Returns true if this error suggests the user needs to take action.
  pub fn requires_user_action(&self) -> bool {
    matches!(
      self,
      Error::InvalidUrl { .. }
        | Error::InvalidPath { .. }
        | Error::FileExists(_)
        | Error::ExistingFiles { .. }
        | Error::FileTooLarge { .. }
        | Error::Authentication { .. }
        | Error::InsufficientSpace { .. }
        | Error::Configuration { .. }
    )
  }

  /// Returns suggested actions for resolving this error.
  pub fn suggested_actions(&self) -> Vec<&'static str> {
    match self {
      Error::InvalidUrl { .. } => vec![
        "Check the URL format",
        "Ensure the protocol is supported (http/https)",
        "Verify the URL is accessible in a browser",
      ],
      Error::FileExists(_) => vec![
        "Use overwrite policy to replace existing files",
        "Choose a different target directory",
        "Rename or move the existing file",
      ],
      Error::FileTooLarge { .. } => vec![
        "Increase the max_file_size limit",
        "Download to a location with more space",
        "Skip this file if not needed",
      ],
      Error::Authentication { .. } => vec![
        "Check your credentials",
        "Verify you have permission to access this redownload",
        "Add authentication headers to the configuration",
      ],
      Error::InsufficientSpace { .. } => vec![
        "Free up disk space",
        "Choose a different download location",
        "Delete unnecessary files",
      ],
      Error::RequestFailed { .. } => vec![
        "Check your internet connection",
        "Verify the server is accessible",
        "Try again later if the server is temporarily down",
      ],
      Error::Timeout { .. } => vec![
        "Increase the timeout duration",
        "Check your internet connection speed",
        "Try downloading during off-peak hours",
      ],
      _ => vec!["Check the error details and try again"],
    }
  }

  /// Returns the error category for grouping similar errors.
  pub fn category(&self) -> ErrorKind {
    match self {
      Error::InvalidUrl { .. } | Error::Validation { .. } => {
        ErrorKind::Validation
      }
      Error::InvalidPath { .. } | Error::FileSystem { .. } => {
        ErrorKind::FileSystem
      }
      Error::RequestFailed { .. } | Error::Timeout { .. } => ErrorKind::Network,
      Error::HttpStatus { .. } => ErrorKind::Http,
      Error::FileExists(_) | Error::ExistingFiles { .. } => {
        ErrorKind::FileConflict
      }
      Error::Authentication { .. } | Error::Certificate { .. } => {
        ErrorKind::Authentication
      }
      Error::FileTooLarge { .. } | Error::InsufficientSpace { .. } => {
        ErrorKind::Storage
      }
      Error::TaskFailed { .. } => ErrorKind::Task,
      Error::Configuration { .. } => ErrorKind::Configuration,
      _ => ErrorKind::Other,
    }
  }
}

/// Categories for grouping related errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
  Validation,
  FileSystem,
  Network,
  Http,
  FileConflict,
  Authentication,
  Storage,
  Task,
  Configuration,
  Other,
}

impl std::fmt::Display for ErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ErrorKind::Validation => write!(f, "Validation"),
      ErrorKind::FileSystem => write!(f, "File System"),
      ErrorKind::Network => write!(f, "Network"),
      ErrorKind::Http => write!(f, "HTTP"),
      ErrorKind::FileConflict => write!(f, "File Conflict"),
      ErrorKind::Authentication => write!(f, "Authentication"),
      ErrorKind::Storage => write!(f, "Storage"),
      ErrorKind::Task => write!(f, "Task"),
      ErrorKind::Configuration => write!(f, "Configuration"),
      ErrorKind::Other => write!(f, "Other"),
    }
  }
}

// Automatic conversions from common error types
impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
    let url = err
      .url()
      .map(|u| u.to_string())
      .unwrap_or_else(|| "unknown".to_string());

    if err.is_timeout() {
      Error::Timeout { url, duration: 30 } // Default timeout assumption
    } else if err.is_redirect() {
      Error::TooManyRedirects {
        url,
        count: 10,
        limit: 10,
      } // Default assumption
    } else {
      Error::RequestFailed { url, download: err }
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Error::FileSystem {
      message: err.to_string(),
    }
  }
}

impl From<tokio::task::JoinError> for Error {
  fn from(err: tokio::task::JoinError) -> Self {
    Error::TaskFailed {
      index: 0, // Can't determine index from JoinError
      reason: err.to_string(),
    }
  }
}

impl From<url::ParseError> for Error {
  fn from(e: url::ParseError) -> Self {
    Error::Validation {
      message: format!("URL parse error: {e}"),
    }
  }
}

/// Helper function to format existing files for error display.
fn format_existing_files(files: &HashMap<String, PathBuf>) -> String {
  if files.len() <= 3 {
    files
      .values()
      .map(|p| p.display().to_string())
      .collect::<Vec<_>>()
      .join(", ")
  } else {
    format!(
      "{}, and {} more",
      files
        .values()
        .take(3)
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(", "),
      files.len() - 3
    )
  }
}

/// Helper function to format multiple errors for display.
fn format_multiple_errors(errors: &[Error]) -> String {
  if errors.len() <= 3 {
    errors
      .iter()
      .map(|e| e.to_string())
      .collect::<Vec<_>>()
      .join("; ")
  } else {
    format!(
      "{}; and {} more errors",
      errors
        .iter()
        .take(3)
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("; "),
      errors.len() - 3
    )
  }
}
