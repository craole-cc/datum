// -- Filesystem-related Definition (types/filesystem/definition.rs) -- //
use super::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  // ================ File Errors ================
  #[error("Failed to create file '{path}' {context}")]
  FileCreate {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  #[error("Failed to read file '{path}' {context}")]
  FileRead {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  #[error("Failed to write file '{path}' {context}")]
  FileWrite {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  #[error("Failed to delete file '{path}' {context}")]
  FileDelete {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  #[error("Failed to copy file from '{from}' to '{to}' {context}")]
  FileCopy {
    from: PathBuf,
    to: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  #[error("Failed to move file from '{from}' to '{to}' {context}")]
  FileMove {
    from: PathBuf,
    to: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  // ================ Directory Errors ================
  #[error("Failed to create directory '{path}' {context}")]
  DirCreate {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  #[error("Failed to read directory '{path}' {context}")]
  DirRead {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  #[error("Failed to delete directory '{path}' {context}")]
  DirDelete {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  // ================ Common Errors ================
  #[error("Path '{path}' not found {context}")]
  NotFound {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  #[error("Permission denied for '{path}' {context}")]
  PermissionDenied {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  #[error("Path '{path}' already exists {context}")]
  AlreadyExists {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  #[error("Generic filesystem error for '{path}' {context}")]
  Other {
    path: PathBuf,
    context: String,
    #[source]
    source: io::Error,
  },

  // ================ Multiple Errors ================
  #[error("Multiple errors occurred")]
  Multiple {
    count: usize,
    context: String,
    errors: Vec<Error>,
  },

  // ================ Generic/Context Errors ================
  #[error("IO operation failed: {context}")]
  Context { context: String },
}

/// Error category for better error organization and handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
  FileCreate,
  FileRead,
  FileWrite,
  FileDelete,
  FileCopy,
  FileMove,
  DirCreate,
  DirRead,
  DirDelete,
  NotFound,
  PermissionDenied,
  AlreadyExists,
  Other,
  Multiple,
  Context,
}

impl From<&Error> for Category {
  fn from(err: &Error) -> Self {
    match err {
      Error::FileCreate { .. } => Category::FileCreate,
      Error::FileRead { .. } => Category::FileRead,
      Error::FileWrite { .. } => Category::FileWrite,
      Error::FileDelete { .. } => Category::FileDelete,
      Error::FileCopy { .. } => Category::FileCopy,
      Error::FileMove { .. } => Category::FileMove,
      Error::DirCreate { .. } => Category::DirCreate,
      Error::DirRead { .. } => Category::DirRead,
      Error::DirDelete { .. } => Category::DirDelete,
      Error::NotFound { .. } => Category::NotFound,
      Error::PermissionDenied { .. } => Category::PermissionDenied,
      Error::AlreadyExists { .. } => Category::AlreadyExists,
      Error::Other { .. } => Category::Other,
      Error::Multiple { .. } => Category::Multiple,
      Error::Context { .. } => Category::Context,
    }
  }
}

/// Result type alias for data operations
pub type Result<T> = result::Result<T, Error>;
