// examples/simple_demo.rs - Library usage without fancy feature

use erks::{Category, Context, Diagnostic, Error, Result, Severity};
use std::{fmt::Debug, fs::File, io::Read, path::Path};

fn main() -> Result<()> {
  println!("=== Erks (Basic Usage Example - Fancy feature disabled) ===\n");

  demonstrate_error_methods();

  Ok(())
}

/// Main library function that demonstrates error handling
/// This works with or without miette's "fancy" feature
pub fn process_file<P: AsRef<Path> + Debug>(path: P) -> Result<String> {
  // Method 1: Using context trait (most ergonomic)
  let mut file = File::open(&path)
    .context(format!("could not open file at {:?}", path.as_ref()))?;

  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .context(format!("could not read contents of {:#?}", path.as_ref()))?;

  Ok(contents)
}

/// Alternative using constructor methods
pub fn process_file_with_constructors<P: AsRef<Path> + Debug>(
  path: P,
) -> Result<String> {
  let path_ref = path.as_ref();

  // Method 2: Using constructor methods
  let mut file = File::open(path_ref)
    .map_err(|source| Error::file_read(source, path_ref))?;

  let mut contents = String::new();
  file.read_to_string(&mut contents).map_err(|source| {
    Error::file_read_with_context(source, path_ref, "reading file contents")
  })?;

  Ok(contents)
}

/// Manual error construction example
pub fn process_file_manual<P: AsRef<Path> + Debug>(path: P) -> Result<String> {
  let path_buf = path.as_ref().to_path_buf();

  // Method 3: Manual construction (most verbose but most control)
  let mut file = File::open(&path_buf).map_err(|source| Error::FileRead {
    source,
    path: path_buf.clone(),
    context: format!("opening file for processing: {path_buf:?}"),
  })?;

  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .map_err(|source| Error::FileRead {
      source,
      path: path_buf.clone(),
      context: format!("reading contents from: {path_buf:?}"),
    })?;

  Ok(contents)
}

/// Utility function to display error information without fancy formatting
pub fn display_error_info(error: &Error) {
  println!("Error Information:");
  println!("  Message: {error}");

  if let Some(severity) = error.severity() {
    println!("  Severity: {severity:?}");
  }

  // Category is not optional - it always returns a value
  let category = error.category();
  println!("  Category: {category:?}");

  if let Some(code) = error.code() {
    println!("  Code: {code}");
  }

  if let Some(help) = error.help() {
    println!("  Help: {help}");
  }

  if let Some(url) = error.url() {
    println!("  Documentation: {url}");
  }

  // Display the error chain
  let mut source = std::error::Error::source(error);
  let mut level = 1;
  while let Some(err) = source {
    println!("  Caused by (level {level}): {err}");
    source = std::error::Error::source(err);
    level += 1;
  }
}

/// Function to demonstrate all error creation methods
pub fn demonstrate_error_methods() {
  let test_paths = [
    "/nonexistent/file.txt",
    "/data/config.toml",
    "/data/test.tsv",
  ];

  for path in &test_paths {
    println!("Testing with path: {path}");

    // Test method 1: Context trait
    match process_file(path) {
      Ok(contents) => {
        println!("  ✅ Context method: Read {} bytes", contents.len())
      }
      Err(e) => {
        println!("  ❌ Context method failed:");
        display_error_info(&e);
      }
    }

    // Test method 2: Constructor methods
    match process_file_with_constructors(path) {
      Ok(contents) => {
        println!("  ✅ Constructor method: Read {} bytes", contents.len())
      }
      Err(e) => {
        println!("  ❌ Constructor method failed:");
        display_error_info(&e);
      }
    }

    // Test method 3: Manual construction
    match process_file_manual(path) {
      Ok(contents) => {
        println!("  ✅ Manual method: Read {} bytes", contents.len())
      }
      Err(e) => {
        println!("  ❌ Manual method failed:");
        display_error_info(&e);
      }
    }

    println!(); // Empty line between paths
  }
}

/// Error handling that works well in libraries
pub fn handle_result<T>(result: Result<T>) -> Option<T> {
  match result {
    Ok(value) => Some(value),
    Err(error) => {
      // Log the error without panicking - good for library code
      eprintln!("Operation failed: {error}");

      // Optionally log additional diagnostic info
      if let Some(help) = error.help() {
        eprintln!("Suggestion: {help}");
      }

      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::ErrorKind;

  #[test]
  fn test_process_file_methods() {
    let nonexistent_path = "/definitely/does/not/exist.txt";

    // All methods should fail gracefully
    assert!(process_file(nonexistent_path).is_err());
    assert!(process_file_with_constructors(nonexistent_path).is_err());
    assert!(process_file_manual(nonexistent_path).is_err());
  }

  #[test]
  fn test_error_diagnostic_properties() {
    let error = Error::file_read(
      std::io::Error::new(ErrorKind::NotFound, "test error"),
      "/test/path.txt",
    );

    // Test diagnostic properties work without fancy feature
    assert!(error.severity().is_some());
    assert!(error.code().is_some());
    assert!(error.help().is_some());
    assert!(error.url().is_some());

    // Test category (always returns a value)
    let category = error.category();
    assert_eq!(category, Category::Filesystem);

    // Test error message format
    let message = format!("{}", error);
    assert!(message.contains("Failed to read file"));
    assert!(message.contains("/test/path.txt"));
  }

  #[test]
  fn test_context_intelligent_mapping() {
    let io_error = std::io::Error::new(ErrorKind::NotFound, "file not found");
    let result: Result<()> = Err(io_error)
      .context("could not read config at \"/missing/config.toml\"");

    let error = result.unwrap_err();

    // Should be mapped to PathNotFound due to intelligent context mapping
    match error {
      Error::PathNotFound { path, context, .. } => {
        assert_eq!(path.to_string_lossy(), "/missing/config.toml");
        assert!(context.contains("could not read config"));
      }
      other => panic!("Expected PathNotFound, got {:?}", other),
    }
  }

  #[test]
  fn test_error_chain() {
    let error = Error::file_read_with_context(
      std::io::Error::new(ErrorKind::PermissionDenied, "access denied"),
      "/root/secret.txt",
      "loading sensitive configuration",
    );

    // Test that source chain works
    assert!(std::error::Error::source(&error).is_some());

    let source = std::error::Error::source(&error).unwrap();
    assert_eq!(source.to_string(), "access denied");
  }

  #[test]
  fn test_handle_result_utility() {
    // Test successful case
    let success: Result<i32> = Ok(42);
    assert_eq!(handle_result(success), Some(42));

    // Test error case
    let failure: Result<i32> = Err(Error::file_read(
      std::io::Error::new(ErrorKind::NotFound, "not found"),
      "/missing.txt",
    ));
    assert_eq!(handle_result(failure), None);
  }

  #[test]
  fn test_category_always_available() {
    let error = Error::file_read(
      std::io::Error::new(ErrorKind::NotFound, "test error"),
      "/test/path.txt",
    );

    // Category should always be available (not optional)
    let category = error.category();
    assert_eq!(category, Category::Filesystem);
  }
}
