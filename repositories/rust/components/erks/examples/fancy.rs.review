// examples/fancier.rs - Demo with fancy miette features
// Run with: cargo run --example erks_fancier --features fancy

use erks::{
  Context, Diagnostic, Error, IntoDiagnostic, MietteHandlerOpts, Report,
  Result, WrapErr, miette, set_hook, set_panic_hook,
};
use std::{
  error::Error as StdError,
  fs::{File, read_to_string},
  io::{Error as IOError, ErrorKind as IOErrorKind, Read},
  path::{Path, PathBuf},
  result::Result as StdResult,
};

fn main() -> Result<()> {
  // Install the panic handler for pretty error reporting
  // set_panic_hook();

  // Install custom miette handler for beautiful error display
  set_hook(Box::new(|_| {
    Box::new(
      MietteHandlerOpts::new()
        .terminal_links(true)
        .unicode(true)
        .context_lines(1)
        .build(),
    )
  }))
  .unwrap();

  // Run our example and handle errors with pretty printing
  println!("\n=== Erks (Advanced Usage Example - Fancy feature enabled) ===\n");

  // Method 1: Manual construction
  test_manual_construction()?;

  // Method 2: Constructor methods
  test_constructor_methods()?;

  // Method 3: Context trait with intelligent mapping
  test_context_trait()?;

  // Method 4: Demonstrate error chaining
  test_error_chaining()?;

  // Method 5: Error conversion
  test_error_conversion()?;

  println!("✅ All error handling methods demonstrated successfully!");
  Ok(())
}

fn test_error_conversion() -> Result<()> {
  // Test converting standard errors
  match demonstrate_error_conversion() {
    Ok(_) => println!("   ✓ No errors occurred"),
    Err(e) => {
      println!("   ✗ Conversion error (expected): {e}");
      display_error_details(&e, "     ");
    }
  }

  // Test custom error creation
  match demonstrate_custom_errors() {
    Ok(_) => println!("   ✓ No errors occurred"),
    Err(e) => {
      println!("   ✗ Custom error (expected): {e}");
      display_error_details(&e, "     ");
    }
  }

  println!();
  Ok(())
}

fn test_manual_construction() -> Result<()> {
  println!("1. Manual Error Construction:");

  let path = "/data/test.tsv";
  let file_result = File::open(path).map_err(|source| Error::FileRead {
    source,
    path: PathBuf::from(path),
    context: String::from("for manual data analysis"),
  });

  match file_result {
    Ok(_) => println!("   ✓ File opened successfully"),
    Err(e) => {
      println!("   ✗ Error occurred (expected):");
      display_error_details(&e, "     ");

      // Show the pretty formatted version
      println!("     🎨 Pretty formatted:");
      println!("{:?}", Report::new(e));
    }
  }
  Ok(())
}

fn test_constructor_methods() -> Result<()> {
  println!("2. Constructor Methods:");

  let path = Path::new("/data/config.toml");

  // Try to open file and use constructor if it fails
  match File::open(path) {
    Ok(_) => println!("   ✓ File opened successfully"),
    Err(src) => {
      // Basic constructor
      let err = Error::file_read(src, path);
      println!("   ✗ Basic constructor: {err}");

      // Constructor with context
      let source = IOErrorKind::InvalidFilename;
      let err_with_context = Error::file_read_with_context(
        source.into(),
        path,
        "while loading application configuration",
      );
      println!("   ✗ With context: {err_with_context}");

      display_error_details(&err_with_context, "     ");

      // Show the pretty formatted version
      println!("     🎨 Pretty formatted:");
      println!("{:?}", Report::new(err_with_context));
    }
  }
  println!();
  Ok(())
}

fn test_context_trait() -> Result<()> {
  println!("3. Context Trait (Intelligent Mapping):");
  // This will fail and be automatically mapped to PathNotFound
  // let result = File::open(path).context(
  //   "to check intelligent context detection",
  // );

  let path = Path::new("/data/missing.txt");
  let result = File::open(path)
    .context_with_path(path, "to check intelligent context detection");

  match result {
    Ok(_) => println!("   ✓ File opened successfully"),
    Err(e) => {
      println!("   ✗ Error occurred (expected): {e}");

      let error_type = match e {
        Error::PathNotFound { .. } => "PathNotFound (intelligently mapped!)",
        Error::PathPermissionDenied { .. } => "PathPermissionDenied",
        Error::FileRead { .. } => "FileRead",
        Error::Context { .. } => "Context (fallback)",
        _ => "Other",
      };

      println!("   🧠 Intelligently mapped to: {error_type}");
      display_error_details(&e, "     ");

      // Show category mapping
      println!("   📂 Category: {:?}", e.category());

      // Show the pretty formatted version
      println!("     🎨 Pretty formatted:");
      println!("{:?}", Report::new(e));
    }
  }
  Ok(())
}

fn test_error_chaining() -> Result<()> {
  println!("\n4. Error Chaining with Context:");

  let result = File::open("/secure/protected.txt")
    .context("failed to access secure configuration");

  match result {
    Ok(_) => println!("   ✓ File opened successfully"),
    Err(e) => {
      println!("   ✗ Chained error occurred (expected):");
      println!("   🔗 Error chain:");

      // Display the full error chain
      let mut current_error: &dyn StdError = &e;
      let mut level = 1;

      loop {
        println!("     Level {}: {}", level, current_error);

        match current_error.source() {
          Some(source) => {
            current_error = source;
            level += 1;
          }
          None => break,
        }
      }

      // Show pretty formatted version
      println!("   🎨 Pretty formatted chain:");
      println!("{:?}", Report::new(e));
    }
  }
  Ok(())
}

fn display_error_details(error: &Error, indent: &str) {
  println!("{}📋 Error Details:", indent);

  if let Some(code) = error.code() {
    println!("{}  - Code: {}", indent, code);
  }

  if let Some(severity) = error.severity() {
    println!("{}  - Severity: {:?}", indent, severity);
  }

  println!("{}  - Category: {:?}", indent, error.category());

  if let Some(help) = error.help() {
    println!("{}  - Help: {}", indent, help);
  }

  if let Some(url) = error.url() {
    println!("{}  - Documentation: {}", indent, url);
  }
}

// Demonstrate converting different error types using Context
fn demonstrate_error_conversion() -> Result<()> {
  println!("\n5. Converting Standard Errors with Context:");

  // Convert a standard library error using context
  let _content = read_to_string("/nonexistent/app.conf")
    .context("failed to load application configuration")?;

  Ok(())
}

// Demonstrate creating ad-hoc errors with your Error enum
fn demonstrate_custom_errors() -> Result<()> {
  println!("6. Custom Error Creation:");

  let config_valid = false;

  if !config_valid {
    return Err(Error::Context {
      source: None,
      context: "Configuration validation failed - invalid format detected"
        .to_string(),
    });
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pretty_error_display() {
    let err = Error::file_read(
      IOError::new(IOErrorKind::NotFound, "file not found"),
      "/data/test.csv",
    );

    // Test that the error displays nicely
    let error_string = format!("{err}");
    assert!(error_string.contains("Failed to read file"));
    assert!(error_string.contains("/data/test.csv"));
  }

  #[test]
  fn test_miette_integration() {
    let err = Error::file_read(
      IOError::new(IOErrorKind::PermissionDenied, "access denied"),
      "/root/secret.txt",
    );

    // Test miette diagnostic properties using erks re-exports
    assert!(err.severity().is_some());
    assert!(err.code().is_some());
    assert!(err.help().is_some());
    assert!(err.url().is_some());

    // Test category
    use erks::Category;
    assert_eq!(err.category(), Category::Filesystem);
  }

  #[test]
  fn test_intelligent_context_mapping() {
    let io_error = IOError::new(IOErrorKind::NotFound, "file not found");
    let result: StdResult<(), IOError> = Err(io_error);

    let mapped_error =
      result.context("could not open file at \"/missing/config.toml\"");

    match mapped_error {
      Ok(_) => panic!("Expected error"),
      Err(Error::PathNotFound { path, context, .. }) => {
        assert_eq!(path.to_string_lossy(), "/missing/config.toml");
        assert!(context.contains("could not open file"));
      }
      Err(other) => {
        // Fallback to Context is also acceptable
        println!("Mapped to: {:?}", other);
      }
    }
  }

  #[test]
  fn test_error_chaining() {
    let err = Error::file_read_with_context(
      IOError::new(IOErrorKind::PermissionDenied, "access denied"),
      "/root/secret.txt",
      "loading sensitive configuration",
    );

    // Test that source chain works
    assert!(StdError::source(&err).is_some());

    let source = StdError::source(&err).unwrap();
    assert_eq!(source.to_string(), "access denied");
  }

  #[test]
  fn test_context_functionality() {
    let result: StdResult<(), IOError> =
      Err(IOError::new(IOErrorKind::NotFound, "not found"));

    let with_context = result.context("failed to read config");

    assert!(with_context.is_err());
    let error = with_context.unwrap_err();

    // Should have a source error
    assert!(StdError::source(&error).is_some());
  }
}
