// src/main.rs - Complete example showing pretty error printing

use erks::{Context, Error, Result};
use miette::{Diagnostic, IntoDiagnostic, WrapErr};
use std::fs::File;
use std::io::Read;
fn main() {
  // Install miette's panic handler for pretty error reporting
  miette::set_panic_hook();

  // Run our example and handle errors with pretty printing
  if let Err(err) = run_example() {
    eprintln!("{:?}", miette::Report::new(err));
    std::process::exit(1);
  }
}

fn run_example() -> Result<()> {
  println!("Testing error creation methods...\n");

  // Method 1: Manual construction
  test_manual_construction()?;

  // Method 2: Constructor methods
  test_constructor_methods()?;

  // Method 3: Context trait
  test_context_trait()?;

  println!("All methods work!");
  Ok(())
}

fn test_manual_construction() -> Result<()> {
  println!("1. Manual construction:");

  let path = "/data/test.tsv";
  let file_result = File::open(path).map_err(|source| Error::FileRead {
    source,
    path: std::path::PathBuf::from(path),
    context: String::from("for manual inspection"),
  });

  match file_result {
    Ok(_) => println!("   ✓ File opened successfully"),
    Err(e) => {
      println!("   ✗ Error occurred (expected): {e}");
      println!("   📋 Error details:");
      println!("      - Code: {:?}", e.code().map(|c| c.to_string()));
      println!("      - Severity: {:?}", e.severity());
      println!("      - Help: {:?}", e.help().map(|h| h.to_string()));
      println!("      - URL: {:?}", e.url().map(|u| u.to_string()));
    }
  }
  println!();
  Ok(())
}

fn test_constructor_methods() -> Result<()> {
  println!("2. Constructor methods:");

  let path = "/data/config.toml";

  // Try to open file and use constructor if it fails
  match File::open(path) {
    Ok(_) => println!("   ✓ File opened successfully"),
    Err(source) => {
      let err = Error::file_read(source, path);
      println!("   ✗ Error occurred (expected): {err}");

      // Also try with context
      let source2 =
        std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
      let err_with_context = Error::file_read_with_context(
        source2,
        path,
        "while loading application configuration",
      );
      println!("   ✗ With context: {err_with_context}");
    }
  }
  println!();
  Ok(())
}

fn test_context_trait() -> Result<()> {
  println!("3. Context trait (intelligent mapping):");

  // This will fail and be automatically mapped to PathNotFound
  let result = File::open("/data/missing.txt")
    .context("could not read config at \"/data/missing.txt\"");

  match result {
    Ok(_) => println!("   ✓ File opened successfully"),
    Err(e) => {
      println!("   ✗ Error occurred (expected): {e}");
      println!(
        "   🧠 Intelligently mapped to: {:?}",
        match e {
          Error::PathNotFound { .. } => "PathNotFound",
          Error::PathPermissionDenied { .. } => "PathPermissionDenied",
          Error::Context { .. } => "Context",
          _ => "Other",
        }
      );
    }
  }
  println!();
  Ok(())
}

// Demonstrate different error scenarios
fn demonstrate_error_scenarios() -> Result<()> {
  println!("Demonstrating different error scenarios:\n");

  // Scenario 1: File not found
  let _content = std::fs::read_to_string("/nonexistent/file.txt")
    .context("reading application configuration")?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pretty_error_display() {
    let err = Error::file_read(
      std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
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
      std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "access denied",
      ),
      "/root/secret.txt",
    );

    // Test miette diagnostic properties
    use miette::Diagnostic;
    assert!(err.severity().is_some());
    assert!(err.code().is_some());
    assert!(err.help().is_some());
    assert!(err.url().is_some());
  }
}
