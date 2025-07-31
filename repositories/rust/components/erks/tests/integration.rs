use erks::Error;
use miette::{Diagnostic, Severity};
use std::path::PathBuf;

#[test]
fn file_read_diagnostic_properties() {
  let err = Error::FileRead {
    source: std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
    path: PathBuf::from("myfile.txt"),
    context: "couldn't load".to_string(),
  };

  // severity is Warning
  let sev = err.severity().unwrap();
  assert_eq!(sev, Severity::Warning);

  // diagnostic code
  let code = err.code().unwrap().to_string();
  assert_eq!(code, "erks::file_read");

  // help message
  let help = err.help().unwrap().to_string();
  assert_eq!(help, "ensure the file exists and is readable");

  // documentation URL
  let url = err.url().unwrap().to_string();
  assert_eq!(url, "https://example.com/docs/file-read");
}

#[test]
fn file_create_has_warning_severity() {
  let err = Error::FileCreate {
    source: std::io::Error::new(
      std::io::ErrorKind::PermissionDenied,
      "no perms",
    ),
    path: PathBuf::from("newfile.txt"),
    context: "oops".to_string(),
  };

  // Medium severity maps to Warning
  let sev = err.severity().unwrap();
  assert_eq!(sev, Severity::Warning);
}

#[test]
fn context_variant_has_advice_severity() {
  let err = Error::Context {
    source: None,
    context: "just telling".to_string(),
  };

  // severity is Advice
  let sev = err.severity().unwrap();
  assert_eq!(sev, Severity::Advice);
}

#[test]
fn path_already_exists_has_advice_severity_and_correct_code() {
  let err = Error::PathAlreadyExists {
    source: std::io::Error::new(std::io::ErrorKind::AlreadyExists, "exists"),
    path: PathBuf::from("exists.txt"),
    context: "duplicate".to_string(),
  };

  // advice-level hint
  let sev = err.severity().unwrap();
  assert_eq!(sev, Severity::Advice);

  let code = err.code().unwrap().to_string();
  assert_eq!(code, "erks::path_already_exists");

  let help = err.help().unwrap().to_string();
  assert_eq!(help, "choose a different name or remove the existing item");
}

#[test]
fn constructor_methods_work() {
  let source1 = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
  let source2 = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
  let path = PathBuf::from("test.txt");

  // Full constructor with context
  let err1 =
    Error::file_read_with_context(source1, path.clone(), "custom context");
  match err1 {
    Error::FileRead {
      source: _,
      path: p,
      context,
    } => {
      assert_eq!(p, path);
      assert_eq!(context, "custom context");
    }
    _ => panic!("Wrong error variant"),
  }

  // Convenience constructor
  let err2 = Error::file_read(source2, &path);
  match err2 {
    Error::FileRead {
      source: _,
      path: p,
      context,
    } => {
      assert_eq!(p, path);
      assert_eq!(context, "");
    }
    _ => panic!("Wrong error variant"),
  }
}

#[test]
fn context_trait_intelligent_mapping() {
  use erks::Context;
  use std::fs::File;

  // This would normally fail, but we can test the error mapping logic
  let result: Result<(), erks::Error> = Err(std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "file not found",
  ))
  .context("could not read config at \"/path/to/config.toml\"");

  match result.unwrap_err() {
    Error::PathNotFound { path, context, .. } => {
      assert_eq!(path, PathBuf::from("/path/to/config.toml"));
      assert_eq!(context, "could not read config at \"/path/to/config.toml\"");
    }
    other => panic!("Expected PathNotFound, got {other:?}"),
  }
}
