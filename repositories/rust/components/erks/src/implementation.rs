use crate::prelude::internal::*;

impl Error {
  /// Extract the underlying IO error from any variant
  pub fn get_io_error(&self) -> Option<&std::io::Error> {
    match self {
      Self::Context { .. } => None, // No IO error in context variant

      // All filesystem variants
      Self::FileRead { source, .. }
      | Self::FileCreate { source, .. }
      | Self::FileWrite { source, .. }
      | Self::FileDelete { source, .. }
      | Self::FileCopy { source, .. }
      | Self::FileMove { source, .. }
      | Self::DirCreate { source, .. }
      | Self::DirRead { source, .. }
      | Self::DirDelete { source, .. }
      | Self::PathNotFound { source, .. }
      | Self::PathPermissionDenied { source, .. }
      | Self::PathAlreadyExists { source, .. } => Some(source),

      // Network variants
      Self::NetworkConnection { source, .. } => Some(source),

      // Resource variants
      Self::OutOfMemory { source, .. } => Some(source),

      // Input variants
      Self::InvalidInput { source, .. } => Some(source),
    }
  }

  /// Auto-detect category from the underlying IO error (for validation)
  pub fn auto_category(&self) -> Category {
    match self {
      Self::Context { .. } => Category::Undefined,
      _ => {
        if let Some(io_error) = self.get_io_error() {
          Category::from_error_kind(io_error.kind())
        } else {
          Category::Undefined
        }
      }
    }
  }

  /// Auto-detect severity from the underlying IO error (for validation)
  pub fn auto_severity(&self) -> crate::Severity {
    match self {
      Self::Context { .. } => crate::Severity::Low,
      _ => {
        if let Some(io_error) = self.get_io_error() {
          crate::Severity::from_error_kind(io_error.kind())
        } else {
          crate::Severity::Medium
        }
      }
    }
  }

  /// Validate that manual category makes sense for this error
  pub fn validate_category(&self) -> bool {
    if let Some(io_error) = self.get_io_error() {
      Category::validate_for_io_error(self.category(), io_error)
    } else {
      true // Context errors can be any category
    }
  }

  /// Validate that manual severity makes sense for this error
  pub fn validate_severity(&self) -> bool {
    if let Some(io_error) = self.get_io_error() {
      // Convert miette severity back to domain severity for comparison
      let manual_miette = self.severity().unwrap_or(miette::Severity::Warning);
      let manual_domain = match manual_miette {
        miette::Severity::Advice => crate::Severity::Low,
        miette::Severity::Warning => crate::Severity::Medium,
        miette::Severity::Error => crate::Severity::High, // Could be High or Critical
      };

      crate::Severity::validate_for_io_error(manual_domain, io_error)
    } else {
      true // Context errors can be any severity
    }
  }

  /// Development helper - analyze categorization choices
  pub fn debug_categorization(&self) {
    let manual_cat = self.category();
    let auto_cat = self.auto_category();
    let auto_sev = self.auto_severity();

    println!("=== Error Analysis: {self:?} ===");

    if let Some(io_error) = self.get_io_error() {
      println!("IO Error Kind: {:?}", io_error.kind());
    }

    println!(
      "Category - Manual: {:?}, Auto: {:?}, Valid: {}",
      manual_cat,
      auto_cat,
      self.validate_category()
    );
    println!("Severity - Auto-suggested: {auto_sev:?}");

    if !self.validate_category() {
      println!("⚠️  Category might need review!");
    }
  }
}

#[cfg(test)]
mod error_validation_tests {
  use super::*;
  use std::io;

  #[test]
  fn test_error_category_validation() {
    // Test case that should validate correctly
    let error = Error::file_read(
      io::Error::new(io::ErrorKind::NotFound, "file not found"),
      "/tmp/test.txt",
    );

    assert!(
      error.validate_category(),
      "FileRead with NotFound should have valid category"
    );

    // Test auto-detection
    assert_eq!(error.auto_category(), Category::Filesystem);
  }

  #[test]
  fn test_error_severity_validation() {
    let error = Error::path_permission_denied(
      io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"),
      "/etc/shadow",
    );

    // Should validate because High severity makes sense for PermissionDenied
    assert!(error.validate_severity());
  }

  #[test]
  fn test_context_error_validation() {
    let error = Error::Context {
      source: None,
      context: "test context".to_string(),
    };

    // Context errors should always validate (no IO error to compare against)
    assert!(error.validate_category());
    assert!(error.validate_severity());
    assert!(error.get_io_error().is_none());
  }

  #[test]
  #[ignore] // Run with: cargo test debug_all_variants -- --ignored
  fn debug_all_error_variants() {
    let test_cases = vec![
      Error::file_read(
        io::Error::new(io::ErrorKind::NotFound, "test"),
        "/tmp/file",
      ),
      Error::path_permission_denied(
        io::Error::new(io::ErrorKind::PermissionDenied, "test"),
        "/etc/shadow",
      ),
      // Add your new variants here:
      // Error::network_connection(io::Error::new(io::ErrorKind::ConnectionRefused, "test"), "localhost:8080"),
      // Error::out_of_memory(io::Error::new(io::ErrorKind::OutOfMemory, "test"), Some(1024)),
      // Error::invalid_input(io::Error::new(io::ErrorKind::InvalidInput, "test"), "bad data"),
    ];

    for error in test_cases {
      error.debug_categorization();
      println!("---");
    }
  }
}
