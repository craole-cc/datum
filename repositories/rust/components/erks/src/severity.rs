use crate::prelude::internal::*;

/// Domain-specific severity levels that map to miette severities
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
  /// Low impact - informational or advice
  Low,
  /// Medium impact - warnings that should be addressed
  Medium,
  /// High impact - errors that prevent operation
  High,
  /// Critical impact - system-threatening errors
  Critical,
}

impl Severity {
  /// Convert domain severity to miette severity
  pub fn to_miette(self) -> MietteSeverity {
    match self {
      Severity::Low => MietteSeverity::Advice,
      Severity::Medium => MietteSeverity::Warning,
      Severity::High => MietteSeverity::Error,
      Severity::Critical => MietteSeverity::Error,
    }
  }

  /// Auto-detect severity from std::io::ErrorKind
  pub fn from_error_kind(kind: ErrorKind) -> Self {
    use ErrorKind::*;

    match kind {
      // Critical system errors
      OutOfMemory => Severity::Critical,

      // High severity - operation blocking
      PermissionDenied | ConnectionRefused | ConnectionReset
      | ConnectionAborted | TimedOut => Severity::High,

      // Medium severity - recoverable issues
      NotFound | AlreadyExists | AddrInUse | AddrNotAvailable
      | InvalidInput | InvalidData | UnexpectedEof => Severity::Medium,

      // Low severity - non-blocking issues
      WouldBlock | Interrupted => Severity::Low,

      // Network issues (unstable variants)
      #[cfg(feature = "unstable")]
      HostUnreachable | NetworkUnreachable | NetworkDown => Severity::High,

      // Resource issues (unstable variants)
      #[cfg(feature = "unstable")]
      StorageFull | QuotaExceeded | ResourceBusy | ExecutableFileBusy => {
        Severity::Critical
      }

      // Filesystem issues (unstable variants)
      #[cfg(feature = "unstable")]
      FilesystemLoop
      | StaleNetworkFileHandle
      | NotSeekable
      | FileTooLarge
      | CrossesDevices
      | TooManyLinks
      | InvalidFilename => Severity::Medium,

      // System issues (unstable variants)
      #[cfg(feature = "unstable")]
      Unsupported | ArgumentListTooLong => Severity::Medium,

      // Concurrency issues (unstable variants)
      #[cfg(feature = "unstable")]
      Deadlock | InProgress => Severity::High,

      // Everything else
      Other | _ => Severity::Medium,
    }
  }

  /// Validate if a manual severity makes sense for the given IO error
  pub fn validate_for_io_error(
    manual_severity: Severity,
    io_error: &std::io::Error,
  ) -> bool {
    let auto_severity = Severity::from_error_kind(io_error.kind());
    manual_severity == auto_severity
      || Severity::is_reasonable_override(manual_severity, auto_severity)
  }

  /// Check if a severity override is reasonable/intentional
  pub fn is_reasonable_override(manual: Severity, auto: Severity) -> bool {
    match (manual, auto) {
      // Same severity is always fine
      (a, b) if a == b => true,

      // One level difference is usually reasonable
      (Severity::Low, Severity::Medium) | (Severity::Medium, Severity::Low) => {
        true
      }
      (Severity::Medium, Severity::High)
      | (Severity::High, Severity::Medium) => true,
      (Severity::High, Severity::Critical)
      | (Severity::Critical, Severity::High) => true,

      // Context-specific downgrades (e.g., treating permission errors as medium in some contexts)
      (Severity::Medium, Severity::High) => true,

      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::ErrorKind;

  #[test]
  fn test_severity_from_error_kind() {
    assert_eq!(
      Severity::from_error_kind(ErrorKind::OutOfMemory),
      Severity::Critical
    );
    assert_eq!(
      Severity::from_error_kind(ErrorKind::PermissionDenied),
      Severity::High
    );
    assert_eq!(
      Severity::from_error_kind(ErrorKind::NotFound),
      Severity::Medium
    );
    assert_eq!(
      Severity::from_error_kind(ErrorKind::WouldBlock),
      Severity::Low
    );
  }

  #[test]
  fn test_severity_to_miette() {
    assert_eq!(Severity::Low.to_miette(), miette::Severity::Advice);
    assert_eq!(Severity::Medium.to_miette(), miette::Severity::Warning);
    assert_eq!(Severity::High.to_miette(), miette::Severity::Error);
    assert_eq!(Severity::Critical.to_miette(), miette::Severity::Error);
  }

  #[test]
  fn test_severity_validation() {
    let io_error = std::io::Error::new(ErrorKind::PermissionDenied, "test");

    // Should validate - matches auto-detection
    assert!(Severity::validate_for_io_error(Severity::High, &io_error));

    // Should validate - reasonable override
    assert!(Severity::validate_for_io_error(Severity::Medium, &io_error));

    // Should not validate - unreasonable override
    assert!(!Severity::validate_for_io_error(Severity::Low, &io_error));
  }
}
