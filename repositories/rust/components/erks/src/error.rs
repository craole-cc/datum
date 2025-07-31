use crate::prelude::internal::*;

/// Central error type for the `erks` crate
#[derive(Debug, thiserror::Error, macros::Error)]
pub enum Error {
  /// Low severity (Advice)
  #[severity(Low)]
  #[category(Undefined)]
  #[doc = include_str!("../docs/context.md")]
  #[error("{context}")]
  #[url("https://example.com/docs/context")]
  Context {
    #[source]
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
    context: String,
  },

  /// Medium severity (Warning)
  #[severity(Medium)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/file-read.md")]
  #[error("Failed to read file '{path}' {context}")]
  #[url("https://example.com/docs/file-read")]
  #[help("ensure the file exists and is readable")]
  FileRead {
    #[source]
    source: IOError,
    path: PathBuf,
    context: String,
  },

  #[severity(Medium)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/file-create.md")]
  #[error("Failed to create file '{path}' {context}")]
  #[url("https://example.com/docs/file-create")]
  #[help("check directory permissions and existence")]
  FileCreate {
    #[source]
    source: IOError,
    path: PathBuf,
    context: String,
  },

  #[severity(Medium)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/file-write.md")]
  #[error("Failed to write file '{path}' {context}")]
  #[url("https://example.com/docs/file-write")]
  #[help("verify disk space and write permissions")]
  FileWrite {
    #[source]
    source: IOError,
    path: PathBuf,
    context: String,
  },

  #[severity(Medium)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/file-delete.md")]
  #[error("Failed to delete file '{path}' {context}")]
  #[url("https://example.com/docs/file-delete")]
  #[help("ensure no handles are open and you have permissions")]
  FileDelete {
    #[source]
    source: IOError,
    path: PathBuf,
    context: String,
  },

  #[severity(Medium)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/file-copy.md")]
  #[error("Failed to copy file from '{from}' to '{to}' {context}")]
  #[url("https://example.com/docs/file-copy")]
  #[help("check both paths exist and you have read/write access")]
  FileCopy {
    #[source]
    source: IOError,
    from: PathBuf,
    to: PathBuf,
    context: String,
  },

  #[severity(Medium)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/file-move.md")]
  #[error("Failed to move file from '{from}' to '{to}' {context}")]
  #[url("https://example.com/docs/file-move")]
  #[help("ensure destination directory exists")]
  FileMove {
    #[source]
    source: IOError,
    from: PathBuf,
    to: PathBuf,
    context: String,
  },

  #[severity(Medium)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/dir-create.md")]
  #[error("Failed to create directory '{path}' {context}")]
  #[url("https://example.com/docs/dir-create")]
  #[help("make sure parent directories exist")]
  DirCreate {
    #[source]
    source: IOError,
    path: PathBuf,
    context: String,
  },

  #[severity(Medium)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/dir-read.md")]
  #[error("Failed to read directory '{path}' {context}")]
  #[url("https://example.com/docs/dir-read")]
  #[help("verify directory exists and is accessible")]
  DirRead {
    #[source]
    source: IOError,
    path: PathBuf,
    context: String,
  },

  #[severity(Medium)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/dir-delete.md")]
  #[error("Failed to delete directory '{path}' {context}")]
  #[url("https://example.com/docs/dir-delete")]
  #[help("ensure it's empty before deletion")]
  DirDelete {
    #[source]
    source: IOError,
    path: PathBuf,
    context: String,
  },

  #[severity(Medium)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/path-not-found.md")]
  #[error("Path '{path}' not found {context}")]
  #[url("https://example.com/docs/not-found")]
  #[help("check for typos or ensure the file/directory exists")]
  PathNotFound {
    #[source]
    source: IOError,
    path: PathBuf,
    context: String,
  },

  #[severity(High)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/path-permission-denied.md")]
  #[error("Permission denied for '{path}' {context}")]
  #[url("https://example.com/docs/path-permission-denied")]
  #[help("adjust file system permissions")]
  PathPermissionDenied {
    #[source]
    source: IOError,
    path: PathBuf,
    context: String,
  },

  #[severity(Low)]
  #[category(Filesystem)]
  #[doc = include_str!("../docs/path-already-exists.md")]
  #[error("Path '{path}' already exists {context}")]
  #[url("https://example.com/docs/already-exists")]
  #[help("choose a different name or remove the existing item")]
  PathAlreadyExists {
    #[source]
    source: IOError,
    path: PathBuf,
    context: String,
  },

  // Network errors
  #[severity(High)]
  #[category(Network)]
  #[error("Network connection failed {context}")]
  NetworkConnection {
    #[source]
    source: IOError,
    endpoint: String,
    context: String,
  },

  // Resource errors
  #[severity(Critical)]
  #[category(Resource)]
  #[error("Out of memory {context}")]
  OutOfMemory {
    #[source]
    source: IOError,
    requested_bytes: Option<usize>,
    context: String,
  },

  // Input validation errors
  #[severity(Medium)]
  #[category(Input)]
  #[error("Invalid input data {context}")]
  InvalidInput {
    #[source]
    source: IOError,
    input: String,
    context: String,
  },
}
