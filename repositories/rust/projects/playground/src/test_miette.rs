use crate::*;
use thiserror::Error;
// Pure miette approach - using diagnostic derive with automatic location
#[derive(Error, Debug, Diagnostic)]
pub enum Error {
  #[error("Database connection failed: {reason}")]
  #[diagnostic(
    code(myapp::db::connection_failed),
    help("Check database credentials and connectivity"),
    url("https://docs.myapp.com/db-troubleshooting")
  )]
  DatabaseConnection { reason: String },

  #[error("Validation failed: {message}")]
  #[diagnostic(
    code(myapp::validation::failed),
    help("Ensure input meets validation requirements")
  )]
  Validation { message: String },

  #[error("Configuration error: {details}")]
  #[diagnostic(
    code(myapp::config::error),
    help("Check configuration file syntax and required fields")
  )]
  Configuration { details: String },
}

// Miette's built-in location capture using macros
macro_rules! db_error {
  ($reason:expr) => {
    miette!(
      code = "myapp::db::connection_failed",
      help = "Check database credentials and connectivity",
      "Database connection failed at {}:{}: {}",
      file!(),
      line!(),
      $reason
    )
  };
}

macro_rules! validation_error {
  ($message:expr) => {
    miette!(
      code = "myapp::validation::failed",
      help = "Ensure input meets validation requirements",
      "Validation failed at {}:{}: {}",
      file!(),
      line!(),
      $message
    )
  };
}

macro_rules! config_error {
  ($details:expr) => {
    miette!(
      code = "myapp::config::error",
      help = "Check configuration file syntax and required fields",
      "Configuration error at {}:{}: {}",
      file!(),
      line!(),
      $details
    )
  };
}

// For more advanced cases, create a macro that captures full context
macro_rules! error_here {
  ($error_type:ident, $($args:tt)*) => {
    Error::$error_type {
      $($args)*
    }.with_source_location(file!(), line!(), column!())
  };
}

// Extension trait to add location info to existing errors
trait WithLocation {
  fn with_source_location(
    self,
    file: &'static str,
    line: u32,
    column: u32,
  ) -> miette::Report;
}

impl<E> WithLocation for E
where
  E: Into<miette::Report>,
{
  fn with_source_location(
    self,
    file: &'static str,
    line: u32,
    column: u32,
  ) -> miette::Report {
    self
      .into()
      .wrap_err(format!("Error originated at {file}:{line}:{column}"))
  }
}

// Business logic functions demonstrating location capture

// Sync function using pure miette
fn connect_to_database() -> Result<String> {
  // Using miette::bail! with location info
  miette::bail!(
    "Database connection failed at {}:{} - Connection timeout after 30 seconds",
    file!(),
    line!()
  );
}

// Async function - track_caller doesn't work here, but miette macros do!
async fn async_connect_to_database() -> Result<String> {
  // This works perfectly in async contexts
  Err(db_error!("Async connection timeout"))?
}

async fn async_validate_user_input(input: &str) -> Result<()> {
  if input.is_empty() {
    return Err(validation_error!("Input cannot be empty"))?;
  }

  if input.len() > 100 {
    // Different line, shows different location
    return Err(validation_error!("Input too long"))?;
  }

  Ok(())
}

async fn async_parse_config() -> Result<String> {
  // Using the config_error! macro for direct error creation with location
  Err(config_error!("Invalid configuration format"))?
}

async fn async_process_file(filename: &str) -> Result<String> {
  // Convert std errors to miette with location using IntoDiagnostic
  tokio::fs::read_to_string(filename)
    .await
    .into_diagnostic()
    .wrap_err(format!("Failed to read file at {}:{}", file!(), line!()))?;

  Ok("File processed".to_string())
}

// Chain multiple async operations with location tracking
async fn complex_async_operation() -> Result<()> {
  async_connect_to_database().await.wrap_err_with(|| {
    format!("Database connection failed at {}:{}", file!(), line!())
  })?;

  async_validate_user_input("").await.wrap_err_with(|| {
    format!("Validation failed at {}:{}", file!(), line!())
  })?;

  async_parse_config().await.wrap_err_with(|| {
    format!("Config parsing failed at {}:{}", file!(), line!())
  })?;

  async_process_file("nonexistent.txt")
    .await
    .wrap_err_with(|| {
      format!("File processing failed at {}:{}", file!(), line!())
    })?;

  Ok(())
}

// Pure miette approach for capturing multiple locations in error chains
async fn operation_with_context_chain() -> Result<()> {
  let result = async_connect_to_database()
    .await
    .wrap_err("Step 1: Database initialization")
    .wrap_err_with(|| format!("Called from {}:{}", file!(), line!()));

  if let Err(e) = result {
    return Err(
      e.wrap_err("Step 2: Fallback connection attempt")
        .wrap_err(format!("Final error at {}:{}", file!(), line!())),
    );
  }

  Ok(())
}

// Helper function to demonstrate miette's built-in source location features
fn create_detailed_error() -> Result<()> {
  // Miette can automatically include source information
  let source_info = format!("{}:{}:{}", file!(), line!(), column!());

  Err(miette!(
    code = "myapp::detailed::error",
    help = "This error includes detailed source location information",
    "Detailed error occurred at source location: {}",
    source_info
  ))
}

// Using miette's ensure! macro for assertions with location
fn validate_with_ensure(value: i32) -> Result<()> {
  ensure!(
    value > 0,
    "Value must be positive, got {} at {}:{}",
    value,
    file!(),
    line!()
  );

  ensure!(
    value < 100,
    "Value must be less than 100, got {} at {}:{}",
    value,
    file!(),
    line!()
  );

  Ok(())
}

pub async fn main() -> Result<()> {
  miette::set_panic_hook();

  println!("=== Pure Miette Location Capture Demo ===\n");

  println!("1. Sync function with miette::bail!:");
  if let Err(e) = connect_to_database() {
    eprintln!("{e:?}\n");
  }

  println!("2. Async database connection error:");
  if let Err(e) = async_connect_to_database().await {
    eprintln!("{e:?}\n");
  }

  println!("3. Async validation error:");
  if let Err(e) = async_validate_user_input("").await {
    eprintln!("{e:?}\n");
  }

  println!("4. Async config parsing error:");
  if let Err(e) = async_parse_config().await {
    eprintln!("{e:?}\n");
  }

  println!("5. Async file processing with wrap_err:");
  if let Err(e) = async_process_file("nonexistent.txt").await {
    eprintln!("{e:?}\n");
  }

  println!("6. Complex async operation chain:");
  if let Err(e) = complex_async_operation().await {
    eprintln!("{e:?}\n");
  }

  println!("7. Async operation with context chain:");
  if let Err(e) = operation_with_context_chain().await {
    eprintln!("{e:?}\n");
  }

  println!("8. Detailed error with source info:");
  if let Err(e) = create_detailed_error() {
    eprintln!("{e:?}\n");
  }

  println!("9. Validation with ensure! macro:");
  if let Err(e) = validate_with_ensure(-5) {
    eprintln!("{e:?}\n");
  }

  if let Err(e) = validate_with_ensure(150) {
    eprintln!("{e:?}\n");
  }

  // Demonstrate same function called from different locations
  println!("10. Same async function, different call sites:");

  // First call site
  if let Err(e) = async_validate_user_input("this string is way too long to pass validation because it exceeds the maximum allowed length").await {
        eprintln!("First call: {e:?}\n");
    }

  // Second call site - will show this location in wrap_err
  if let Err(e) = async_validate_user_input("").await.wrap_err_with(|| {
    format!("Called from second location at {}:{}", file!(), line!())
  }) {
    eprintln!("Second call: {e:?}\n");
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_async_location_capture() {
    let err = async_connect_to_database().await.unwrap_err();
    let err_string = format!("{err:?}");

    // Should contain source location information
    assert!(err_string.contains("src/main.rs"));
    println!("Captured error: {err_string}");
  }

  #[test]
  fn test_miette_macro_location() {
    let err = validation_error!("test error");
    let err_string = format!("{err:?}");

    // Should contain the test function location
    assert!(err_string.contains("test_miette_macro_location"));
    println!("Macro error: {err_string}");
  }

  #[test]
  fn test_ensure_macro() {
    let result = validate_with_ensure(-1);
    assert!(result.is_err());

    let err_string = format!("{:?}", result.unwrap_err());
    assert!(err_string.contains("src/main.rs"));
    println!("Ensure error: {err_string}");
  }
}
