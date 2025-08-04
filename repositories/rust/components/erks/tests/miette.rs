#[cfg(test)]
mod tests {
  use erks::*;
  use std::{fs, io};

  // Helper function that returns a standard library error for testing
  fn failing_io_operation() -> io::Result<String> {
    fs::read_to_string("nonexistent_file.txt")
  }

  // Helper function that returns a custom error for testing
  fn custom_error() -> Result<String> {
    Err(miette!("This is a custom error"))
  }

  #[test]
  fn test_with_context_macro() {
    let result = with_context!(failing_io_operation(), "Reading config file");

    assert!(result.is_err());
    let error_string = format!("{:?}", result.unwrap_err());

    // Should contain the context and location
    assert!(error_string.contains("Reading config file"));
    assert!(error_string.contains(file!()));

    println!("with_context test error: {error_string}");
  }

  #[test]
  fn test_with_context_macro_different_locations() {
    // First call site
    let result1 = with_context!(failing_io_operation(), "First location");
    let error1_string = format!("{:?}", result1.unwrap_err());

    // Second call site - should show different line number
    let result2 = with_context!(failing_io_operation(), "Second location");
    let error2_string = format!("{:?}", result2.unwrap_err());

    // Both should contain their respective contexts
    assert!(error1_string.contains("First location"));
    assert!(error2_string.contains("Second location"));

    // Line numbers should be different
    assert_ne!(error1_string, error2_string);

    println!("First location error: {error1_string}");
    println!("Second location error: {error2_string}");
  }

  #[test]
  fn test_with_location_trait() {
    let custom_err = miette!("Test error");
    let located_error =
      custom_err.with_source_location(file!(), line!(), column!());

    let error_string = format!("{located_error:?}");

    // Should contain location information
    assert!(error_string.contains("Error originated at"));
    assert!(error_string.contains(file!()));

    println!("with_location trait test: {error_string}");
  }

  #[test]
  fn test_with_location_trait_chaining() {
    let io_error = failing_io_operation();
    let located_error = io_error
      .into_diagnostic()
      .unwrap_err()
      .with_source_location(file!(), line!(), column!());

    let error_string = format!("{located_error:?}");

    // Should contain both the IO error and location info
    assert!(error_string.contains("Error originated at"));
    assert!(
      error_string.contains("No such file or directory")
        || error_string.contains("cannot find the file")
    );

    println!("Chained location test: {error_string}");
  }

  #[test]
  fn test_error_at_macro() {
    let error = error_at!("Something went wrong with value: {}", 42);
    let error_string = format!("{error:?}");

    // Should contain location and formatted message
    assert!(error_string.contains("Error at"));
    assert!(error_string.contains("Something went wrong with value: 42"));
    assert!(error_string.contains(file!()));

    println!("error_at macro test: {error_string}");
  }

  #[test]
  fn test_diagnostic_error_macro_with_code_and_help() {
    let error = diagnostic_error!(
      code = "test::error::with_help",
      help = "Try checking your input parameters",
      "Database connection failed: {}",
      "timeout"
    );

    let error_string = format!("{error:?}");

    // Should contain all diagnostic information
    assert!(error_string.contains("test::error::with_help"));
    assert!(error_string.contains("Try checking your input parameters"));
    assert!(error_string.contains("Database connection failed: timeout"));
    assert!(error_string.contains("Error at"));
    assert!(error_string.contains(file!()));

    println!("diagnostic_error with help test: {error_string}");
  }

  #[test]
  fn test_diagnostic_error_macro_code_only() {
    let error = diagnostic_error!(
      code = "test::error::code_only",
      "Validation failed for input: {}",
      "empty_string"
    );

    let error_string = format!("{error:?}");

    // Should contain code and message
    assert!(error_string.contains("test::error::code_only"));
    assert!(error_string.contains("Validation failed for input: empty_string"));
    assert!(error_string.contains("Error at"));

    println!("diagnostic_error code only test: {error_string}");
  }

  #[test]
  fn test_with_context_preserves_original_error() {
    let result = with_context!(failing_io_operation(), "File operation");
    let error = result.unwrap_err();
    let error_string = format!("{error:?}");

    // Should contain both context and original IO error details
    assert!(error_string.contains("File operation"));
    assert!(
      error_string.contains("No such file or directory")
        || error_string.contains("cannot find the file")
    );

    println!("Context preservation test: {error_string}");
  }

  #[test]
  fn test_multiple_context_layers() {
    let result: Result<String> =
      with_context!(failing_io_operation(), "Reading config")
        .and_then(|_| Err(miette!("Processing failed")))
        .wrap_err("Application startup failed");

    let error = result.unwrap_err();
    let error_string = format!("{error:?}");

    // Should show error chain
    assert!(error_string.contains("Reading config"));
    assert!(error_string.contains("Application startup failed"));

    println!("Multiple context test: {error_string}");
  }

  #[test]
  fn test_location_accuracy() {
    // Test that different lines produce different location info
    let error1 = error_at!("First error");
    let error2 = error_at!("Second error");

    let error1_string = format!("{error1:?}");
    let error2_string = format!("{error2:?}");

    // Should have different line numbers
    assert_ne!(error1_string, error2_string);
    assert!(error1_string.contains("First error"));
    assert!(error2_string.contains("Second error"));

    println!("Location accuracy test 1: {error1_string}");
    println!("Location accuracy test 2: {error2_string}");
  }

  #[tokio::test]
  async fn test_with_context_async() {
    async fn async_failing_operation() -> io::Result<String> {
      tokio::fs::read_to_string("nonexistent_async_file.txt").await
    }

    let result =
      with_context!(async_failing_operation().await, "Async file read");

    assert!(result.is_err());
    let error_string = format!("{:?}", result.unwrap_err());

    // Should work in async context
    assert!(error_string.contains("Async file read"));
    assert!(error_string.contains(file!()));

    println!("Async context test: {error_string}");
  }

  #[test]
  fn test_integration_with_standard_miette_features() {
    // Test that our extensions work with standard miette features
    let error = diagnostic_error!(
      code = "integration::test",
      help = "This tests integration with miette",
      "Integration test failed"
    );

    // Should be able to use standard miette methods
    let wrapped = error.wrap_err("Outer context");
    let error_string = format!("{wrapped:?}");

    assert!(error_string.contains("integration::test"));
    assert!(error_string.contains("Outer context"));
    assert!(error_string.contains("Integration test failed"));

    println!("Integration test: {error_string}");
  }
}
