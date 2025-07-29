// -- Implementation (types/filesystem/implementation.rs) -- //
use super::*;

impl Error {
  /// Return the category of this error.
  pub fn category(&self) -> Category {
    Category::from(self)
  }

  /// Get the severity level of this error
  pub fn severity(&self) -> Severity {
    match self {
      // Critical errors - system failures
      Self::DirDelete { .. } | Self::FileDelete { .. } => Severity::Critical,

      // High severity - operation failures
      Self::FileRead { .. } | Self::FileWrite { .. } => Severity::High,

      // Medium severity - data issues that stop processing
      Self::FileCreate { .. }
      | Self::FileCopy { .. }
      | Self::FileMove { .. }
      | Self::DirRead { .. }
      | Self::PermissionDenied { .. }
      | Self::Other { .. } => Severity::Medium,

      // Low severity - warnings and minor issues
      Self::DirCreate { .. } | Self::AlreadyExists { .. } => Severity::Low,

      // Multiple errors inherit highest severity
      Self::Multiple { errors, .. } => errors
        .iter()
        .map(|e| e.severity())
        .max()
        .unwrap_or(Severity::Medium),

      // Default to medium for other errors
      _ => Severity::Medium,
    }
  }

  /// Get the underlying `io::ErrorKind`
  pub fn kind(&self) -> io::ErrorKind {
    match self {
      Self::FileCreate { source, .. }
      | Self::FileRead { source, .. }
      | Self::FileWrite { source, .. }
      | Self::FileDelete { source, .. }
      | Self::FileCopy { source, .. }
      | Self::FileMove { source, .. }
      | Self::DirCreate { source, .. }
      | Self::DirRead { source, .. }
      | Self::DirDelete { source, .. }
      | Self::NotFound { source, .. }
      | Self::PermissionDenied { source, .. }
      | Self::AlreadyExists { source, .. }
      | Self::Other { source, .. } => source.kind(),
      Self::Multiple { errors, .. } => errors
        .iter()
        .max_by_key(|e| e.severity())
        .map(|e| e.kind())
        .unwrap_or(io::ErrorKind::Other),
      _ => io::ErrorKind::Other,
    }
  }

  /// Checks whether this file system error is recoverable (e.g. safe to retry).
  pub fn is_recoverable(&self) -> bool {
    match self {
      Self::PermissionDenied { .. }
      | Self::AlreadyExists { .. }
      | Self::FileMove { .. }
      | Self::FileCopy { .. }
      | Self::FileCreate { .. } => true,

      Self::Multiple { errors, .. } => {
        errors.iter().any(|e| e.is_recoverable())
      }

      _ => false,
    }
  }

  /// Whether this error should be logged as warning
  pub fn is_warning(&self) -> bool {
    matches!(self.severity(), Severity::Low)
  }

  /// Combine multiple errors into one
  pub fn combine(errors: Vec<Error>) -> Option<Self> {
    match errors.len() {
      0 => None,
      1 => Some(errors.into_iter().next().unwrap()),
      n => Some(Error::Multiple {
        count: n,
        context: "Combined multiple errors".to_string(),
        errors,
      }),
    }
  }

  // ================ Constructor Methods ================
  pub fn with_context<S: Into<String>>(message: S) -> Self {
    Self::Context {
      context: message.into(),
    }
  }

  /// Get the primary path involved in the error
  pub fn path(&self) -> &Path {
    match self {
      Self::FileCreate { path, .. }
      | Self::FileRead { path, .. }
      | Self::FileWrite { path, .. }
      | Self::FileDelete { path, .. }
      | Self::DirCreate { path, .. }
      | Self::DirRead { path, .. }
      | Self::DirDelete { path, .. }
      | Self::NotFound { path, .. }
      | Self::PermissionDenied { path, .. }
      | Self::AlreadyExists { path, .. }
      | Self::Other { path, .. } => path,
      Self::FileCopy { from, .. } | Self::FileMove { from, .. } => from,
      _ => Path::new(""),
    }
  }

  /// Get all paths involved in the error (for operations involving multiple paths)
  pub fn paths(&self) -> Vec<&Path> {
    match self {
      Self::FileCopy { from, to, .. } | Self::FileMove { from, to, .. } => {
        vec![from, to]
      }
      _ => vec![self.path()],
    }
  }

  /// Get the context string without formatting
  pub fn context(&self) -> &str {
    match self {
      Self::FileCreate { context, .. }
      | Self::FileRead { context, .. }
      | Self::FileWrite { context, .. }
      | Self::FileDelete { context, .. }
      | Self::FileCopy { context, .. }
      | Self::FileMove { context, .. }
      | Self::DirCreate { context, .. }
      | Self::DirRead { context, .. }
      | Self::DirDelete { context, .. }
      | Self::NotFound { context, .. }
      | Self::PermissionDenied { context, .. }
      | Self::AlreadyExists { context, .. }
      | Self::Other { context, .. }
      | Self::Multiple { context, .. }
      | Self::Context { context, .. } => context,
    }
  }

  /// Get a pretty-formatted error message
  pub fn pretty(&self) -> String {
    match self {
      Error::Multiple {
        count,
        context,
        errors,
      } => {
        let mut out = format!("{count} errors occurred: {context}\n");
        for (i, err) in errors.iter().enumerate() {
          out += &format!("  {}. {}\n", i + 1, err);
        }
        out
      }
      _ => self.to_string(), // Use default Display
    }
  }
}

/// Generates both domain‐level and root‐level constructors for each fs variant.
macro_rules! define_fs_wrappers {
    (
        $(
            ($Variant:ident, [$($field:ident),+], $desc:expr)
        ),* $(,)?
    ) => {
        paste! {
            // 1) Domain‐level ctors on types::fs::Error
            impl Error {
                $(
                    #[track_caller]
                    pub fn [<$Variant:snake>]< $([<$field:camel Arg>]: Into<PathBuf>),+ >(
                        source: io::Error,
                        $($field: [<$field:camel Arg>]),+
                    ) -> Self {
                        let loc = Location::caller();
                        let context = format!(
                            "{} at {}:{}:{}",
                            $desc, loc.file(), loc.line(), loc.column()
                        );
                        Self::$Variant {
                            $($field: $field.into()),+,
                            context,
                            source,
                        }
                    }

                    pub fn [<$Variant:snake _with_context>]< $([<$field:camel Arg>]: Into<PathBuf>),+, S: Into<String> >(
                        source: io::Error,
                        $($field: [<$field:camel Arg>]),+,
                        context: S
                    ) -> Self {
                        Self::$Variant {
                            $($field: $field.into()),+,
                            context: context.into(),
                            source,
                        }
                    }
                )*
            }

            // 2) Root‐level ctors on crate::Error
            impl crate::Error {
                $(
                    #[track_caller]
                    pub fn [<fs_ $Variant:snake>]< $([<$field:camel Arg>]: Into<PathBuf>),+ >(
                        source: io::Error,
                        $($field: [<$field:camel Arg>]),+
                    ) -> Self {
                        Self::FileSystem(
                            Error::[<$Variant:snake>](source, $($field),+)
                        )
                    }

                    pub fn [<fs_ $Variant:snake _with_context>]< $([<$field:camel Arg>]: Into<PathBuf>),+, S: Into<String> >(
                        source: io::Error,
                        $($field: [<$field:camel Arg>]),+,
                        context: S
                    ) -> Self {
                        Self::FileSystem(
                            Error::[<$Variant:snake _with_context>](source, $($field),+, context)
                        )
                    }
                )*
            }
        }
    };
}

// Enumerate all your variants here:
define_fs_wrappers! {
    (FileCreate,       [path],      "failed to create file"),
    (FileRead,         [path],      "failed to read file"),
    (FileWrite,        [path],      "failed to write file"),
    (FileDelete,       [path],      "failed to delete file"),
    (FileCopy,         [from,to],   "failed to copy file"),
    (FileMove,         [from,to],   "failed to move file"),
    (DirCreate,        [path],      "failed to create directory"),
    (DirRead,          [path],      "failed to read directory"),
    (DirDelete,        [path],      "failed to delete directory"),
    (NotFound,         [path],      "path not found"),
    (PermissionDenied, [path],      "permission denied"),
    (AlreadyExists,    [path],      "already exists"),
    (Other,            [path],      "generic filesystem error"),
}
