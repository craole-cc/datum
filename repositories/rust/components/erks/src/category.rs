use crate::prelude::internal::*;

/// Error severity levels for better error handling and logging
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
  /// Undefined error type operations
  Undefined,

  /// Database, Dataframes, etc. interactions
  Data,

  /// Filesystem operations (file/directory access, permissions, structure)
  Filesystem,

  /// Network operations (connections, routing, addresses)
  Network,

  /// Resource limitations (memory, storage, quotas, capacity)
  Resource,

  /// Input validation and data format issues
  Input,

  /// System-level operations and permissions
  System,

  /// Concurrent operations and blocking scenarios
  Concurrency,
}

impl Category {
  /// Categorize a Rust `std::io::ErrorKind` into logical groups
  /// Uses unstable variants when "unstable" feature is enabled
  pub fn from_error_kind(kind: std::io::ErrorKind) -> Self {
    use std::io::ErrorKind::*;

    match kind {
      // Filesystem operations (stable)
      NotFound | AlreadyExists | NotADirectory | IsADirectory
      | DirectoryNotEmpty | ReadOnlyFilesystem => Category::Filesystem,

      // Filesystem operations (unstable)
      #[cfg(feature = "unstable")]
      FilesystemLoop
      | StaleNetworkFileHandle
      | NotSeekable
      | FileTooLarge
      | CrossesDevices
      | TooManyLinks
      | InvalidFilename => Category::Filesystem,

      // Network operations (stable)
      ConnectionRefused | ConnectionReset | ConnectionAborted
      | NotConnected | AddrInUse | AddrNotAvailable | BrokenPipe | TimedOut => {
        Category::Network
      }

      // Network operations (unstable)
      #[cfg(feature = "unstable")]
      HostUnreachable | NetworkUnreachable | NetworkDown => Category::Network,

      // Resource limitations (unstable)
      #[cfg(feature = "unstable")]
      StorageFull | QuotaExceeded | OutOfMemory | ResourceBusy
      | ExecutableFileBusy | ArgumentListTooLong => Category::Resource,

      // Input validation and data issues (stable)
      InvalidInput | InvalidData | UnexpectedEof | WriteZero => Category::Input,

      // System-level operations (stable)
      PermissionDenied | Interrupted => Category::System,

      // System-level operations (unstable)
      #[cfg(feature = "unstable")]
      Unsupported => Category::System,

      // Concurrency and blocking (stable)
      WouldBlock => Category::Concurrency,

      // Concurrency and blocking (unstable)
      #[cfg(feature = "unstable")]
      Deadlock | InProgress => Category::Concurrency,

      // Fallback for unknown variants
      Other | _ => Category::Undefined,
    }
  }

  /// Validate if a manual category makes sense for the given IO error
  pub fn validate_for_io_error(
    manual_category: Category,
    io_error: &std::io::Error,
  ) -> bool {
    let auto_category = Category::from_error_kind(io_error.kind());
    manual_category == auto_category
      || Category::is_reasonable_override(manual_category, auto_category)
  }

  /// Check if a category override is reasonable/intentional
  /// For example: PermissionDenied could be System or Filesystem depending on context
  pub fn is_reasonable_override(manual: Category, auto: Category) -> bool {
    match (manual, auto) {
      // Permission errors can be either System or Filesystem depending on context
      (Category::Filesystem, Category::System) => true,
      (Category::System, Category::Filesystem) => true,

      // Network timeouts might be categorized as Resource limits in some contexts
      (Category::Resource, Category::Network) => true,

      // Input validation errors might come from IO operations
      (Category::Input, Category::Filesystem) => true,

      // Same categories are always fine
      (a, b) if a == b => true,

      // Undefined can override anything (manual decision)
      (Category::Undefined, _) => true,

      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::ErrorKind;

  #[test]
  fn test_filesystem_categorization() {
    assert_eq!(
      Category::from_error_kind(ErrorKind::NotFound),
      Category::Filesystem
    );
    assert_eq!(
      Category::from_error_kind(ErrorKind::AlreadyExists),
      Category::Filesystem
    );
    assert_eq!(
      Category::from_error_kind(ErrorKind::IsADirectory),
      Category::Filesystem
    );
    assert_eq!(
      Category::from_error_kind(ErrorKind::PermissionDenied),
      Category::System
    );
  }

  #[test]
  fn test_network_categorization() {
    assert_eq!(
      Category::from_error_kind(ErrorKind::ConnectionRefused),
      Category::Network
    );
    assert_eq!(
      Category::from_error_kind(ErrorKind::TimedOut),
      Category::Network
    );
    assert_eq!(
      Category::from_error_kind(ErrorKind::BrokenPipe),
      Category::Network
    );
  }

  #[test]
  fn test_resource_categorization() {
    #[cfg(feature = "unstable")]
    {
      assert_eq!(
        Category::from_error_kind(ErrorKind::OutOfMemory),
        Category::Resource
      );
      assert_eq!(
        Category::from_error_kind(ErrorKind::StorageFull),
        Category::Resource
      );
      assert_eq!(
        Category::from_error_kind(ErrorKind::QuotaExceeded),
        Category::Resource
      );
    }
  }

  #[test]
  fn test_input_categorization() {
    assert_eq!(
      Category::from_error_kind(ErrorKind::InvalidInput),
      Category::Input
    );
    assert_eq!(
      Category::from_error_kind(ErrorKind::InvalidData),
      Category::Input
    );
  }

  #[test]
  fn test_concurrency_categorization() {
    assert_eq!(
      Category::from_error_kind(ErrorKind::WouldBlock),
      Category::Concurrency
    );

    #[cfg(feature = "unstable")]
    {
      assert_eq!(
        Category::from_error_kind(ErrorKind::Deadlock),
        Category::Concurrency
      );
      assert_eq!(
        Category::from_error_kind(ErrorKind::InProgress),
        Category::Concurrency
      );
    }
  }

  #[test]
  fn test_system_categorization() {
    assert_eq!(
      Category::from_error_kind(ErrorKind::PermissionDenied),
      Category::System
    );
    assert_eq!(
      Category::from_error_kind(ErrorKind::Interrupted),
      Category::System
    );

    #[cfg(feature = "unstable")]
    assert_eq!(
      Category::from_error_kind(ErrorKind::Unsupported),
      Category::System
    );
  }

  #[test]
  fn test_undefined_fallback() {
    // Test that Other falls back to Undefined
    assert_eq!(
      Category::from_error_kind(ErrorKind::Other),
      Category::Undefined
    );
  }
}
