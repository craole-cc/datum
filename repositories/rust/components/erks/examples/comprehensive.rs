// examples/comprehensive.rs - Demo with all error variants
// Run with: cargo run --example erks_comprehensive --features fancy

use erks::{
  Context, Diagnostic, Error, IntoDiagnostic, MietteHandlerOpts,
  MietteSeverity, Report, Result, WrapErr, miette, set_hook, set_panic_hook,
};
use std::{
  error::Error as StdError,
  fs::{
    File, copy, create_dir, read_to_string, remove_dir, remove_file, rename,
  },
  io::{Error as IOError, ErrorKind as IOErrorKind, Read, Write},
  path::{Path, PathBuf},
  result::Result as StdResult,
};

fn main() -> Result<()> {
  // Install the panic handler for pretty error reporting
  // set_panic_hook();

  // Install custom miette handler for beautiful error display
  // set_hook(Box::new(|_| {
  //   Box::new(
  //     MietteHandlerOpts::new()
  //       .terminal_links(true)
  //       .unicode(true)
  //       .context_lines(1)
  //       .build(),
  //   )
  // }))
  // .unwrap();

  println!("\n=== Erks (Comprehensive Error Variants Demo) ===\n");

  // Demonstrate all error variants
  test_context_errors()?;
  test_filesystem_errors()?;
  test_network_errors()?;
  test_resource_errors()?;
  test_input_validation_errors()?;
  test_error_chaining_comprehensive()?;
  test_intelligent_context_mapping()?;

  println!("✅ All error variants demonstrated successfully!");
  Ok(())
}

fn test_context_errors() -> Result<()> {
  println!("1. Context Errors (Low Severity - Advice):");

  // Manual context error creation
  let result = demonstrate_context_error();
  match result {
    Ok(_) => println!("   ✓ No errors occurred"),
    Err(e) => {
      println!("   ✗ Context error (expected): {e}");
      display_error_details(&e, "     ");
      println!("     🎨 Pretty formatted:");
      println!("{:?}", Report::new(e));
    }
  }
  println!();
  Ok(())
}

fn test_filesystem_errors() -> Result<()> {
  println!("2. Filesystem Errors (Medium/High Severity):");

  // File operations
  test_file_read_error()?;
  test_file_create_error()?;
  test_file_write_error()?;
  test_file_delete_error()?;
  test_file_copy_error()?;
  test_file_move_error()?;

  // Directory operations
  test_dir_create_error()?;
  test_dir_read_error()?;
  test_dir_delete_error()?;

  // Path-specific errors
  test_path_not_found_error()?;
  test_path_permission_denied_error()?;
  test_path_already_exists_error()?;

  Ok(())
}

fn test_network_errors() -> Result<()> {
  println!("3. Network Errors (High Severity):");

  let result = demonstrate_network_error();
  match result {
    Ok(_) => println!("   ✓ No network errors occurred"),
    Err(e) => {
      println!("   ✗ Network error (expected): {e}");
      display_error_details(&e, "     ");
      println!("     🎨 Pretty formatted:");
      println!("{:?}", Report::new(e));
    }
  }
  println!();
  Ok(())
}

fn test_resource_errors() -> Result<()> {
  println!("4. Resource Errors (Critical Severity):");

  let result = demonstrate_memory_error();
  match result {
    Ok(_) => println!("   ✓ No memory errors occurred"),
    Err(e) => {
      println!("   ✗ Memory error (expected): {e}");
      display_error_details(&e, "     ");
      println!("     🎨 Pretty formatted:");
      println!("{:?}", Report::new(e));
    }
  }
  println!();
  Ok(())
}

fn test_input_validation_errors() -> Result<()> {
  println!("5. Input Validation Errors (Medium Severity):");

  let result = demonstrate_input_validation_error();
  match result {
    Ok(_) => println!("   ✓ No input validation errors occurred"),
    Err(e) => {
      println!("   ✗ Input validation error (expected): {e}");
      display_error_details(&e, "     ");
      println!("     🎨 Pretty formatted:");
      println!("{:?}", Report::new(e));
    }
  }
  println!();
  Ok(())
}

// Individual filesystem error demonstrations
fn test_file_read_error() -> Result<()> {
  println!("   📄 File Read Error:");
  let path = "/nonexistent/data.txt";
  match File::open(path) {
    Ok(_) => println!("     ✓ File opened successfully"),
    Err(source) => {
      let err = Error::FileRead {
        source,
        path: PathBuf::from(path),
        context: "while loading user configuration".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_file_create_error() -> Result<()> {
  println!("   📝 File Create Error:");
  let path = "/root/protected/new_file.txt";
  match File::create(path) {
    Ok(_) => println!("     ✓ File created successfully"),
    Err(source) => {
      let err = Error::FileCreate {
        source,
        path: PathBuf::from(path),
        context: "during backup process initialization".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_file_write_error() -> Result<()> {
  println!("   ✍️ File Write Error:");
  let path = "/readonly/output.log";
  match std::fs::OpenOptions::new().write(true).open(path) {
    Ok(_) => println!("     ✓ File opened for writing successfully"),
    Err(source) => {
      let err = Error::FileWrite {
        source,
        path: PathBuf::from(path),
        context: "while saving application logs".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_file_delete_error() -> Result<()> {
  println!("   🗑️ File Delete Error:");
  let path = "/protected/system_file.conf";
  match remove_file(path) {
    Ok(_) => println!("     ✓ File deleted successfully"),
    Err(source) => {
      let err = Error::FileDelete {
        source,
        path: PathBuf::from(path),
        context: "during cleanup operation".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_file_copy_error() -> Result<()> {
  println!("   📋 File Copy Error:");
  let from = "/source/important.db";
  let to = "/readonly/backup.db";
  match copy(from, to) {
    Ok(_) => println!("     ✓ File copied successfully"),
    Err(source) => {
      let err = Error::FileCopy {
        source,
        from: PathBuf::from(from),
        to: PathBuf::from(to),
        context: "during database backup".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_file_move_error() -> Result<()> {
  println!("   🚚 File Move Error:");
  let from = "/temp/upload.tmp";
  let to = "/nonexistent/final.dat";
  match rename(from, to) {
    Ok(_) => println!("     ✓ File moved successfully"),
    Err(source) => {
      let err = Error::FileMove {
        source,
        from: PathBuf::from(from),
        to: PathBuf::from(to),
        context: "while processing user upload".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_dir_create_error() -> Result<()> {
  println!("   📁 Directory Create Error:");
  let path = "/root/new_directory";
  match create_dir(path) {
    Ok(_) => println!("     ✓ Directory created successfully"),
    Err(source) => {
      let err = Error::DirCreate {
        source,
        path: PathBuf::from(path),
        context: "for application data storage".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_dir_read_error() -> Result<()> {
  println!("   👀 Directory Read Error:");
  let path = "/private/secrets";
  match std::fs::read_dir(path) {
    Ok(_) => println!("     ✓ Directory read successfully"),
    Err(source) => {
      let err = Error::DirRead {
        source,
        path: PathBuf::from(path),
        context: "while scanning for configuration files".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_dir_delete_error() -> Result<()> {
  println!("   🗂️ Directory Delete Error:");
  let path = "/system/important";
  match remove_dir(path) {
    Ok(_) => println!("     ✓ Directory deleted successfully"),
    Err(source) => {
      let err = Error::DirDelete {
        source,
        path: PathBuf::from(path),
        context: "during cleanup operation".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_path_not_found_error() -> Result<()> {
  println!("   🔍 Path Not Found Error:");
  let path = "/missing/config.toml";
  match File::open(path) {
    Ok(_) => println!("     ✓ Path found successfully"),
    Err(source) => {
      let err = Error::PathNotFound {
        source,
        path: PathBuf::from(path),
        context: "while loading application settings".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_path_permission_denied_error() -> Result<()> {
  println!("   🔒 Path Permission Denied Error:");
  let path = "/root/.ssh/id_rsa";
  match File::open(path) {
    Ok(_) => println!("     ✓ Path accessible"),
    Err(source) => {
      let err = Error::PathPermissionDenied {
        source,
        path: PathBuf::from(path),
        context: "while accessing security credentials".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_path_already_exists_error() -> Result<()> {
  println!("   📂 Path Already Exists Error:");
  let path = "/tmp"; // This should exist on most systems
  match create_dir(path) {
    Ok(_) => println!("     ✓ Directory created successfully"),
    Err(source) => {
      let err = Error::PathAlreadyExists {
        source,
        path: PathBuf::from(path),
        context: "while initializing workspace".to_string(),
      };
      println!("     ✗ {err}");
      display_error_details(&err, "       ");
    }
  }
  Ok(())
}

fn test_error_chaining_comprehensive() -> Result<()> {
  println!("6. Comprehensive Error Chaining:");

  // Create a chain of operations that will fail
  let result = chain_file_operations();
  match result {
    Ok(_) => println!("   ✓ All operations successful"),
    Err(e) => {
      println!("   ✗ Chained error occurred (expected):");
      println!("   🔗 Error chain:");

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

      println!("   🎨 Pretty formatted chain:");
      println!("{:?}", Report::new(e));
    }
  }
  println!();
  Ok(())
}

fn test_intelligent_context_mapping() -> Result<()> {
  println!("7. Intelligent Context Mapping:");

  // Test various IO error kinds and their intelligent mapping
  test_mapping_not_found()?;
  test_mapping_permission_denied()?;
  test_mapping_already_exists()?;

  Ok(())
}

fn test_mapping_not_found() -> Result<()> {
  println!("   🧠 Mapping NotFound -> PathNotFound:");
  let io_error = IOError::new(IOErrorKind::NotFound, "file not found");
  let result: StdResult<(), IOError> = Err(io_error);

  let mapped_error = result.context_with_path(
    Path::new("/missing/config.toml"),
    "could not open configuration file",
  );

  match mapped_error {
    Ok(_) => println!("     ✓ No error occurred"),
    Err(Error::PathNotFound { path, context, .. }) => {
      println!("     ✓ Successfully mapped to PathNotFound");
      println!("       Path: {}", path.display());
      println!("       Context: {}", context);
    }
    Err(other) => {
      println!("     ⚠️ Mapped to different variant: {:?}", other);
    }
  }
  Ok(())
}

fn test_mapping_permission_denied() -> Result<()> {
  println!("   🧠 Mapping PermissionDenied -> PathPermissionDenied:");
  let io_error = IOError::new(IOErrorKind::PermissionDenied, "access denied");
  let result: StdResult<(), IOError> = Err(io_error);

  let mapped_error = result.context_with_path(
    Path::new("/root/secret.txt"),
    "while accessing secure file",
  );

  match mapped_error {
    Ok(_) => println!("     ✓ No error occurred"),
    Err(Error::PathPermissionDenied { path, context, .. }) => {
      println!("     ✓ Successfully mapped to PathPermissionDenied");
      println!("       Path: {}", path.display());
      println!("       Context: {}", context);
    }
    Err(other) => {
      println!("     ⚠️ Mapped to different variant: {:?}", other);
    }
  }
  Ok(())
}

fn test_mapping_already_exists() -> Result<()> {
  println!("   🧠 Mapping AlreadyExists -> PathAlreadyExists:");
  let io_error = IOError::new(IOErrorKind::AlreadyExists, "already exists");
  let result: StdResult<(), IOError> = Err(io_error);

  let mapped_error = result.context_with_path(
    Path::new("/tmp/existing.txt"),
    "while creating new file",
  );

  match mapped_error {
    Ok(_) => println!("     ✓ No error occurred"),
    Err(Error::PathAlreadyExists { path, context, .. }) => {
      println!("     ✓ Successfully mapped to PathAlreadyExists");
      println!("       Path: {}", path.display());
      println!("       Context: {}", context);
    }
    Err(other) => {
      println!("     ⚠️ Mapped to different variant: {:?}", other);
    }
  }
  Ok(())
}

// Helper functions for demonstrating specific error types

fn demonstrate_context_error() -> Result<()> {
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

fn demonstrate_network_error() -> Result<()> {
  // Simulate a network connection failure
  let io_error =
    IOError::new(IOErrorKind::ConnectionRefused, "connection refused");

  Err(Error::NetworkConnection {
    source: io_error,
    endpoint: "api.example.com:443".to_string(),
    context: "while fetching user profile data".to_string(),
  })
}

fn demonstrate_memory_error() -> Result<()> {
  // Simulate an out of memory condition
  let io_error = IOError::new(IOErrorKind::OutOfMemory, "insufficient memory");

  Err(Error::OutOfMemory {
    source: io_error,
    requested_bytes: Some(1_073_741_824), // 1GB
    context: "while loading large dataset".to_string(),
  })
}

fn demonstrate_input_validation_error() -> Result<()> {
  let invalid_input = "not-a-valid-email";
  let io_error = IOError::new(IOErrorKind::InvalidInput, "invalid format");

  Err(Error::InvalidInput {
    source: io_error,
    input: invalid_input.to_string(),
    context: "while validating user registration data".to_string(),
  })
}

fn chain_file_operations() -> Result<()> {
  // This creates a chain of operations that will fail at different levels
  let path = Path::new("/nonexistent/config.json");
  read_to_string(path)
    .context_with_path(path, "for chain of operations test")?;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_all_error_variants() {
    // Test that all variants can be created and have proper diagnostic info
    let errors = vec![
      Error::Context {
        source: None,
        context: "test context".to_string(),
      },
      Error::FileRead {
        source: IOError::new(IOErrorKind::NotFound, "not found"),
        path: PathBuf::from("/test"),
        context: "test".to_string(),
      },
      Error::FileCreate {
        source: IOError::new(IOErrorKind::PermissionDenied, "denied"),
        path: PathBuf::from("/test"),
        context: "test".to_string(),
      },
      Error::FileWrite {
        source: IOError::new(IOErrorKind::WriteZero, "write failed"),
        path: PathBuf::from("/test"),
        context: "test".to_string(),
      },
      Error::FileDelete {
        source: IOError::new(IOErrorKind::PermissionDenied, "denied"),
        path: PathBuf::from("/test"),
        context: "test".to_string(),
      },
      Error::FileCopy {
        source: IOError::new(IOErrorKind::NotFound, "not found"),
        from: PathBuf::from("/src"),
        to: PathBuf::from("/dst"),
        context: "test".to_string(),
      },
      Error::FileMove {
        source: IOError::new(IOErrorKind::NotFound, "not found"),
        from: PathBuf::from("/src"),
        to: PathBuf::from("/dst"),
        context: "test".to_string(),
      },
      Error::DirCreate {
        source: IOError::new(IOErrorKind::AlreadyExists, "exists"),
        path: PathBuf::from("/test"),
        context: "test".to_string(),
      },
      Error::DirRead {
        source: IOError::new(IOErrorKind::PermissionDenied, "denied"),
        path: PathBuf::from("/test"),
        context: "test".to_string(),
      },
      Error::DirDelete {
        source: IOError::new(IOErrorKind::DirectoryNotEmpty, "not empty"),
        path: PathBuf::from("/test"),
        context: "test".to_string(),
      },
      Error::PathNotFound {
        source: IOError::new(IOErrorKind::NotFound, "not found"),
        path: PathBuf::from("/test"),
        context: "test".to_string(),
      },
      Error::PathPermissionDenied {
        source: IOError::new(IOErrorKind::PermissionDenied, "denied"),
        path: PathBuf::from("/test"),
        context: "test".to_string(),
      },
      Error::PathAlreadyExists {
        source: IOError::new(IOErrorKind::AlreadyExists, "exists"),
        path: PathBuf::from("/test"),
        context: "test".to_string(),
      },
      Error::NetworkConnection {
        source: IOError::new(IOErrorKind::ConnectionRefused, "refused"),
        endpoint: "test:80".to_string(),
        context: "test".to_string(),
      },
      Error::OutOfMemory {
        source: IOError::new(IOErrorKind::OutOfMemory, "no memory"),
        requested_bytes: Some(1024),
        context: "test".to_string(),
      },
      Error::InvalidInput {
        source: IOError::new(IOErrorKind::InvalidInput, "invalid"),
        input: "test".to_string(),
        context: "test".to_string(),
      },
    ];

    for error in errors {
      // Each error should have diagnostic properties
      assert!(
        error.code().is_some(),
        "Error should have a code: {:?}",
        error
      );
      assert!(
        error.severity().is_some(),
        "Error should have severity: {:?}",
        error
      );

      // Test display
      let display_string = format!("{}", error);
      assert!(
        !display_string.is_empty(),
        "Error should display properly: {:?}",
        error
      );

      // Test debug
      let debug_string = format!("{:?}", error);
      assert!(
        !debug_string.is_empty(),
        "Error should debug properly: {:?}",
        error
      );
    }
  }

  #[test]
  fn test_error_categories() {
    use erks::Category;

    // Test that filesystem errors have correct category
    let fs_error = Error::FileRead {
      source: IOError::new(IOErrorKind::NotFound, "not found"),
      path: PathBuf::from("/test"),
      context: "test".to_string(),
    };
    assert_eq!(fs_error.category(), Category::Filesystem);

    // Test network error category
    let net_error = Error::NetworkConnection {
      source: IOError::new(IOErrorKind::ConnectionRefused, "refused"),
      endpoint: "test:80".to_string(),
      context: "test".to_string(),
    };
    assert_eq!(net_error.category(), Category::Network);

    // Test resource error category
    let mem_error = Error::OutOfMemory {
      source: IOError::new(IOErrorKind::OutOfMemory, "no memory"),
      requested_bytes: Some(1024),
      context: "test".to_string(),
    };
    assert_eq!(mem_error.category(), Category::Resource);

    // Test input error category
    let input_error = Error::InvalidInput {
      source: IOError::new(IOErrorKind::InvalidInput, "invalid"),
      input: "test".to_string(),
      context: "test".to_string(),
    };
    assert_eq!(input_error.category(), Category::Input);
  }

  #[test]
  fn test_error_severities() {
    use erks::Severity;

    // Test different severity levels
    let context_error = Error::Context {
      source: None,
      context: "test".to_string(),
    };
    assert_eq!(context_error.severity(), Some(MietteSeverity::Advice));

    let fs_error = Error::FileRead {
      source: IOError::new(IOErrorKind::NotFound, "not found"),
      path: PathBuf::from("/test"),
      context: "test".to_string(),
    };
    assert_eq!(fs_error.severity(), Some(MietteSeverity::Warning));

    let perm_error = Error::PathPermissionDenied {
      source: IOError::new(IOErrorKind::PermissionDenied, "denied"),
      path: PathBuf::from("/test"),
      context: "test".to_string(),
    };
    assert_eq!(perm_error.severity(), Some(MietteSeverity::Error));

    let mem_error = Error::OutOfMemory {
      source: IOError::new(IOErrorKind::OutOfMemory, "no memory"),
      requested_bytes: Some(1024),
      context: "test".to_string(),
    };
    assert_eq!(mem_error.severity(), Some(MietteSeverity::Error));
  }

  #[test]
  fn test_constructor_methods() {
    // Test that constructor methods would work (assuming they exist)
    let path = Path::new("/test/file.txt");
    let io_error = IOError::new(IOErrorKind::NotFound, "not found");

    // These would be the constructor method calls if implemented
    // let error = Error::file_read(io_error.clone(), path);
    // let error_with_context = Error::file_read_with_context(
    //     io_error, path, "while loading configuration"
    // );

    // For now, test manual construction
    let manual_error = Error::FileRead {
      source: io_error,
      path: path.to_path_buf(),
      context: "manual construction test".to_string(),
    };

    assert!(manual_error.code().is_some());
    assert!(manual_error.help().is_some());
    assert!(manual_error.url().is_some());
  }
}
