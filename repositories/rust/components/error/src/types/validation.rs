use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Validation and logic errors that don't involve filesystem operations.
///
/// This covers all common validation scenarios organized by category:
/// - Path validation (missing parents, invalid formats, etc.)
/// - Data validation (empty, missing, invalid format)
/// - Configuration validation (missing keys, invalid values)
/// - Business logic validation (constraints, rules, etc.)
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
  // ================ Path Validation Errors ================
  #[error("Path '{path}' has no parent directory")]
  NoParentDirectory { path: PathBuf },

  #[error("Path '{path}' is not absolute")]
  NotAbsolute { path: PathBuf },

  #[error("Path '{path}' is not relative")]
  NotRelative { path: PathBuf },

  #[error("Path '{path}' has invalid extension, expected '{expected}'")]
  InvalidExtension { path: PathBuf, expected: String },

  #[error("Path '{path}' contains invalid characters")]
  InvalidPathChars { path: PathBuf },

  #[error("Path '{path}' exceeds maximum length of {max_length} characters")]
  PathTooLong { path: PathBuf, max_length: usize },

  #[error("Path '{path}' cannot traverse outside allowed directory")]
  PathTraversal { path: PathBuf },

  // ================ Data Validation Errors ================
  #[error("Required field '{field}' is missing")]
  MissingField { field: String },

  #[error("Field '{field}' is empty but required")]
  EmptyField { field: String },

  #[error("Field '{field}' has invalid value: {reason}")]
  InvalidFieldValue { field: String, reason: String },

  #[error("Data is empty: {context}")]
  EmptyData { context: String },

  #[error(
    "Invalid data format for '{field}': expected {expected}, got {actual}"
  )]
  InvalidFormat {
    field: String,
    expected: String,
    actual: String,
  },

  #[error("Value '{value}' is out of range: {constraint}")]
  OutOfRange { value: String, constraint: String },

  #[error("Field '{field}' contains null or invalid UTF-8")]
  InvalidUtf8 { field: String },

  // ================ String/Text Validation Errors ================
  #[error(
    "String '{field}' is too short: minimum {min} characters, got {actual}"
  )]
  TooShort {
    field: String,
    min: usize,
    actual: usize,
  },

  #[error(
    "String '{field}' is too long: maximum {max} characters, got {actual}"
  )]
  TooLong {
    field: String,
    max: usize,
    actual: usize,
  },

  #[error("String '{field}' contains invalid characters: {invalid_chars}")]
  InvalidChars {
    field: String,
    invalid_chars: String,
  },

  #[error("String '{field}' does not match required pattern: {pattern}")]
  PatternMismatch { field: String, pattern: String },

  #[error("Email '{email}' is not valid")]
  InvalidEmail { email: String },

  #[error("URL '{url}' is not valid: {reason}")]
  InvalidUrl { url: String, reason: String },

  // ================ Numeric Validation Errors ================
  #[error("Number '{field}' must be positive, got {value}")]
  NotPositive { field: String, value: String },

  #[error("Number '{field}' must be non-negative, got {value}")]
  Negative { field: String, value: String },

  #[error("Number '{field}' must be between {min} and {max}, got {value}")]
  NumericRange {
    field: String,
    min: String,
    max: String,
    value: String,
  },

  #[error("Number '{field}' cannot be zero")]
  Zero { field: String },

  // ================ Configuration Validation Errors ================
  #[error("Missing required configuration key: '{key}'")]
  MissingConfigKey { key: String },

  #[error("Invalid configuration value for '{key}': {reason}")]
  InvalidConfigValue { key: String, reason: String },

  #[error("Configuration section '{section}' is missing")]
  MissingConfigSection { section: String },

  #[error("Conflicting configuration: {conflict}")]
  ConfigConflict { conflict: String },

  #[error(
    "Configuration key '{key}' is deprecated, use '{replacement}' instead"
  )]
  DeprecatedConfigKey { key: String, replacement: String },

  // ================ Business Logic Validation Errors ================
  #[error("Operation not allowed: {reason}")]
  OperationNotAllowed { reason: String },

  #[error("Constraint violation: {constraint}")]
  ConstraintViolation { constraint: String },

  #[error("Invalid state transition: from '{from}' to '{to}'")]
  InvalidStateTransition { from: String, to: String },

  #[error("Dependency missing: '{dependency}' is required for '{operation}'")]
  MissingDependency {
    dependency: String,
    operation: String,
  },

  #[error("Resource limit exceeded: {limit}")]
  ResourceLimitExceeded { limit: String },

  #[error("Circular dependency detected: {chain}")]
  CircularDependency { chain: String },

  // ================ Collection Validation Errors ================
  #[error("Collection '{name}' is empty but must contain at least one item")]
  EmptyCollection { name: String },

  #[error(
    "Collection '{name}' has too many items: maximum {max}, got {actual}"
  )]
  TooManyItems {
    name: String,
    max: usize,
    actual: usize,
  },

  #[error("Duplicate item found in '{collection}': {item}")]
  DuplicateItem { collection: String, item: String },

  #[error("Required item '{item}' not found in '{collection}'")]
  ItemNotFound { collection: String, item: String },

  #[error("Collection '{name}' contains invalid items: {invalid_items}")]
  InvalidItems {
    name: String,
    invalid_items: Vec<String>,
  },

  // ================ Time/Date Validation Errors ================
  #[error("Date '{field}' is in the past but must be in the future")]
  DateInPast { field: String },

  #[error("Date '{field}' is in the future but must be in the past")]
  DateInFuture { field: String },

  #[error("Date range invalid: start '{start}' is after end '{end}'")]
  InvalidDateRange { start: String, end: String },

  #[error("Timeout value '{field}' is too {direction}: {value}")]
  InvalidTimeout {
    field: String,
    direction: String, // "short" or "long"
    value: String,
  },

  // ================ Multiple Validation Errors ================
  #[error("Multiple validation errors occurred:\n{errors}")]
  Multiple { errors: Vec<Error> },

  // ================ Generic/Fallback Errors ================
  #[error("{message}")]
  Context { message: String },

  #[error("Validation failed: {details}")]
  ValidationFailed { details: String },
}

impl Error {
  /// Check if this error is related to a missing value
  pub fn is_missing(&self) -> bool {
    matches!(
      self,
      Self::MissingField { .. }
        | Self::EmptyField { .. }
        | Self::EmptyData { .. }
        | Self::EmptyCollection { .. }
        | Self::MissingConfigKey { .. }
        | Self::MissingConfigSection { .. }
        | Self::MissingDependency { .. }
        | Self::ItemNotFound { .. }
    )
  }

  /// Check if this error is related to invalid format/structure
  pub fn is_format_error(&self) -> bool {
    matches!(
      self,
      Self::InvalidFormat { .. }
        | Self::InvalidChars { .. }
        | Self::PatternMismatch { .. }
        | Self::InvalidEmail { .. }
        | Self::InvalidUrl { .. }
        | Self::InvalidUtf8 { .. }
        | Self::InvalidExtension { .. }
        | Self::InvalidPathChars { .. }
    )
  }

  /// Check if this error is related to size/length constraints
  pub fn is_size_error(&self) -> bool {
    matches!(
      self,
      Self::TooShort { .. }
        | Self::TooLong { .. }
        | Self::TooManyItems { .. }
        | Self::PathTooLong { .. }
        | Self::OutOfRange { .. }
        | Self::NumericRange { .. }
    )
  }

  /// Get the field name if this error is field-specific
  pub fn field_name(&self) -> Option<&str> {
    match self {
      Self::MissingField { field }
      | Self::EmptyField { field }
      | Self::InvalidFieldValue { field, .. }
      | Self::InvalidFormat { field, .. }
      | Self::TooShort { field, .. }
      | Self::TooLong { field, .. }
      | Self::InvalidChars { field, .. }
      | Self::PatternMismatch { field, .. }
      | Self::NotPositive { field, .. }
      | Self::Negative { field, .. }
      | Self::NumericRange { field, .. }
      | Self::Zero { field }
      | Self::DateInPast { field }
      | Self::DateInFuture { field }
      | Self::InvalidTimeout { field, .. }
      | Self::InvalidUtf8 { field } => Some(field),
      _ => None,
    }
  }

  /// Combine multiple errors into a single error
  pub fn combine(errors: Vec<Error>) -> Self {
    if errors.is_empty() {
      return Self::context("No errors to combine");
    }

    if errors.len() == 1 {
      return errors.into_iter().next().unwrap();
    }

    Self::Multiple { errors }
  }

  // ================ Path Validation Constructors ================

  pub fn no_parent_directory<P: Into<PathBuf>>(path: P) -> Self {
    Self::NoParentDirectory { path: path.into() }
  }

  pub fn not_absolute<P: Into<PathBuf>>(path: P) -> Self {
    Self::NotAbsolute { path: path.into() }
  }

  pub fn not_relative<P: Into<PathBuf>>(path: P) -> Self {
    Self::NotRelative { path: path.into() }
  }

  pub fn invalid_extension<P: Into<PathBuf>, S: Into<String>>(
    path: P,
    expected: S,
  ) -> Self {
    Self::InvalidExtension {
      path: path.into(),
      expected: expected.into(),
    }
  }

  pub fn invalid_path_chars<P: Into<PathBuf>>(path: P) -> Self {
    Self::InvalidPathChars { path: path.into() }
  }

  pub fn path_too_long<P: Into<PathBuf>>(path: P, max_length: usize) -> Self {
    Self::PathTooLong {
      path: path.into(),
      max_length,
    }
  }

  pub fn path_traversal<P: Into<PathBuf>>(path: P) -> Self {
    Self::PathTraversal { path: path.into() }
  }

  // ================ Data Validation Constructors ================

  pub fn missing_field<S: Into<String>>(field: S) -> Self {
    Self::MissingField {
      field: field.into(),
    }
  }

  pub fn empty_field<S: Into<String>>(field: S) -> Self {
    Self::EmptyField {
      field: field.into(),
    }
  }

  pub fn invalid_field_value<S: Into<String>>(field: S, reason: S) -> Self {
    Self::InvalidFieldValue {
      field: field.into(),
      reason: reason.into(),
    }
  }

  pub fn empty_data<S: Into<String>>(context: S) -> Self {
    Self::EmptyData {
      context: context.into(),
    }
  }

  pub fn invalid_format<S: Into<String>>(
    field: S,
    expected: S,
    actual: S,
  ) -> Self {
    Self::InvalidFormat {
      field: field.into(),
      expected: expected.into(),
      actual: actual.into(),
    }
  }

  pub fn out_of_range<S: Into<String>>(value: S, constraint: S) -> Self {
    Self::OutOfRange {
      value: value.into(),
      constraint: constraint.into(),
    }
  }

  pub fn invalid_utf8<S: Into<String>>(field: S) -> Self {
    Self::InvalidUtf8 {
      field: field.into(),
    }
  }

  // ================ String Validation Constructors ================

  pub fn too_short<S: Into<String>>(
    field: S,
    min: usize,
    actual: usize,
  ) -> Self {
    Self::TooShort {
      field: field.into(),
      min,
      actual,
    }
  }

  pub fn too_long<S: Into<String>>(
    field: S,
    max: usize,
    actual: usize,
  ) -> Self {
    Self::TooLong {
      field: field.into(),
      max,
      actual,
    }
  }

  pub fn invalid_chars<S: Into<String>>(field: S, invalid_chars: S) -> Self {
    Self::InvalidChars {
      field: field.into(),
      invalid_chars: invalid_chars.into(),
    }
  }

  pub fn pattern_mismatch<S: Into<String>>(field: S, pattern: S) -> Self {
    Self::PatternMismatch {
      field: field.into(),
      pattern: pattern.into(),
    }
  }

  pub fn invalid_email<S: Into<String>>(email: S) -> Self {
    Self::InvalidEmail {
      email: email.into(),
    }
  }

  pub fn invalid_url<S: Into<String>>(url: S, reason: S) -> Self {
    Self::InvalidUrl {
      url: url.into(),
      reason: reason.into(),
    }
  }

  // ================ Numeric Validation Constructors ================

  pub fn not_positive<S: Into<String>>(field: S, value: S) -> Self {
    Self::NotPositive {
      field: field.into(),
      value: value.into(),
    }
  }

  pub fn negative<S: Into<String>>(field: S, value: S) -> Self {
    Self::Negative {
      field: field.into(),
      value: value.into(),
    }
  }

  pub fn numeric_range<S: Into<String>>(
    field: S,
    min: S,
    max: S,
    value: S,
  ) -> Self {
    Self::NumericRange {
      field: field.into(),
      min: min.into(),
      max: max.into(),
      value: value.into(),
    }
  }

  pub fn zero<S: Into<String>>(field: S) -> Self {
    Self::Zero {
      field: field.into(),
    }
  }

  // ================ Configuration Validation Constructors ================

  pub fn missing_config_key<S: Into<String>>(key: S) -> Self {
    Self::MissingConfigKey { key: key.into() }
  }

  pub fn invalid_config_value<S: Into<String>>(key: S, reason: S) -> Self {
    Self::InvalidConfigValue {
      key: key.into(),
      reason: reason.into(),
    }
  }

  pub fn missing_config_section<S: Into<String>>(section: S) -> Self {
    Self::MissingConfigSection {
      section: section.into(),
    }
  }

  pub fn config_conflict<S: Into<String>>(conflict: S) -> Self {
    Self::ConfigConflict {
      conflict: conflict.into(),
    }
  }

  pub fn deprecated_config_key<S: Into<String>>(
    key: S,
    replacement: S,
  ) -> Self {
    Self::DeprecatedConfigKey {
      key: key.into(),
      replacement: replacement.into(),
    }
  }

  // ================ Business Logic Constructors ================

  pub fn operation_not_allowed<S: Into<String>>(reason: S) -> Self {
    Self::OperationNotAllowed {
      reason: reason.into(),
    }
  }

  pub fn constraint_violation<S: Into<String>>(constraint: S) -> Self {
    Self::ConstraintViolation {
      constraint: constraint.into(),
    }
  }

  pub fn invalid_state_transition<S: Into<String>>(from: S, to: S) -> Self {
    Self::InvalidStateTransition {
      from: from.into(),
      to: to.into(),
    }
  }

  pub fn missing_dependency<S: Into<String>>(
    dependency: S,
    operation: S,
  ) -> Self {
    Self::MissingDependency {
      dependency: dependency.into(),
      operation: operation.into(),
    }
  }

  pub fn resource_limit_exceeded<S: Into<String>>(limit: S) -> Self {
    Self::ResourceLimitExceeded {
      limit: limit.into(),
    }
  }

  pub fn circular_dependency<S: Into<String>>(chain: S) -> Self {
    Self::CircularDependency {
      chain: chain.into(),
    }
  }

  // ================ Collection Validation Constructors ================

  pub fn empty_collection<S: Into<String>>(name: S) -> Self {
    Self::EmptyCollection { name: name.into() }
  }

  pub fn too_many_items<S: Into<String>>(
    name: S,
    max: usize,
    actual: usize,
  ) -> Self {
    Self::TooManyItems {
      name: name.into(),
      max,
      actual,
    }
  }

  pub fn duplicate_item<S: Into<String>>(collection: S, item: S) -> Self {
    Self::DuplicateItem {
      collection: collection.into(),
      item: item.into(),
    }
  }

  pub fn item_not_found<S: Into<String>>(collection: S, item: S) -> Self {
    Self::ItemNotFound {
      collection: collection.into(),
      item: item.into(),
    }
  }

  pub fn invalid_items<S: Into<String>>(
    name: S,
    invalid_items: Vec<S>,
  ) -> Self {
    Self::InvalidItems {
      name: name.into(),
      invalid_items: invalid_items.into_iter().map(Into::into).collect(),
    }
  }

  // ================ Time/Date Constructors ================

  pub fn date_in_past<S: Into<String>>(field: S) -> Self {
    Self::DateInPast {
      field: field.into(),
    }
  }

  pub fn date_in_future<S: Into<String>>(field: S) -> Self {
    Self::DateInFuture {
      field: field.into(),
    }
  }

  pub fn invalid_date_range<S: Into<String>>(start: S, end: S) -> Self {
    Self::InvalidDateRange {
      start: start.into(),
      end: end.into(),
    }
  }

  pub fn invalid_timeout<S: Into<String>>(
    field: S,
    direction: S,
    value: S,
  ) -> Self {
    Self::InvalidTimeout {
      field: field.into(),
      direction: direction.into(),
      value: value.into(),
    }
  }

  // ================ Generic Constructors ================

  pub fn context<S: Into<String>>(message: S) -> Self {
    Self::Context {
      message: message.into(),
    }
  }

  pub fn validation_failed<S: Into<String>>(details: S) -> Self {
    Self::ValidationFailed {
      details: details.into(),
    }
  }
}

/// Result type alias for validation operations
pub type ValidationResult<T> = Result<T, Error>;

/// A validation builder for collecting multiple errors
#[derive(Debug, Default)]
pub struct ValidationBuilder {
  errors: Vec<Error>,
}

impl ValidationBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  /// Add an error to the collection
  pub fn error(&mut self, error: Error) -> &mut Self {
    self.errors.push(error);
    self
  }

  /// Add an error if a condition is true
  pub fn error_if(
    &mut self,
    condition: bool,
    error: impl FnOnce() -> Error,
  ) -> &mut Self {
    if condition {
      self.errors.push(error());
    }
    self
  }

  /// Validate a result and add any error
  pub fn validate<T>(&mut self, result: ValidationResult<T>) -> Option<T> {
    match result {
      Ok(value) => Some(value),
      Err(error) => {
        self.errors.push(error);
        None
      }
    }
  }

  /// Check if there are any errors
  pub fn has_errors(&self) -> bool {
    !self.errors.is_empty()
  }

  /// Get the number of errors
  pub fn error_count(&self) -> usize {
    self.errors.len()
  }

  /// Finish validation and return result
  pub fn finish(self) -> ValidationResult<()> {
    if self.errors.is_empty() {
      Ok(())
    } else {
      Err(Error::combine(self.errors))
    }
  }

  /// Finish validation with a success value
  pub fn finish_with<T>(self, value: T) -> ValidationResult<T> {
    self.finish().map(|_| value)
  }
}

/// Extension trait for `Option<T>` to easily convert `None` to validation errors
pub trait OptionExt<T> {
  fn required(self, field: &str) -> ValidationResult<T>;
  fn required_ctx(self, field: &str, context: &str) -> ValidationResult<T>;
}

impl<T> OptionExt<T> for Option<T> {
  fn required(self, field: &str) -> ValidationResult<T> {
    self.ok_or_else(|| Error::missing_field(field))
  }

  fn required_ctx(self, field: &str, context: &str) -> ValidationResult<T> {
    self.ok_or_else(|| Error::invalid_field_value(field, context))
  }
}

/// Extension trait specifically for `Option<&Path>` from `.parent()` calls
pub trait ParentPathExt {
  fn or_no_parent(self, original_path: &Path) -> ValidationResult<PathBuf>;
}

impl ParentPathExt for Option<&Path> {
  fn or_no_parent(self, original_path: &Path) -> ValidationResult<PathBuf> {
    self
      .map(|p| p.to_path_buf())
      .ok_or_else(|| Error::no_parent_directory(original_path))
  }
}

/// Extension trait for validation helpers on common types
pub trait ValidateExt {
  fn validate_not_empty(&self, field: &str) -> ValidationResult<()>;
  fn validate_length(
    &self,
    field: &str,
    min: Option<usize>,
    max: Option<usize>,
  ) -> ValidationResult<()>;
}

impl ValidateExt for str {
  fn validate_not_empty(&self, field: &str) -> ValidationResult<()> {
    if self.is_empty() {
      Err(Error::empty_field(field))
    } else {
      Ok(())
    }
  }

  fn validate_length(
    &self,
    field: &str,
    min: Option<usize>,
    max: Option<usize>,
  ) -> ValidationResult<()> {
    let len = self.len();

    if let Some(min_len) = min {
      if len < min_len {
        return Err(Error::too_short(field, min_len, len));
      }
    }

    if let Some(max_len) = max {
      if len > max_len {
        return Err(Error::too_long(field, max_len, len));
      }
    }

    Ok(())
  }
}

impl ValidateExt for String {
  fn validate_not_empty(&self, field: &str) -> ValidationResult<()> {
    self.as_str().validate_not_empty(field)
  }

  fn validate_length(
    &self,
    field: &str,
    min: Option<usize>,
    max: Option<usize>,
  ) -> ValidationResult<()> {
    self.as_str().validate_length(field, min, max)
  }
}

impl<T> ValidateExt for Vec<T> {
  fn validate_not_empty(&self, field: &str) -> ValidationResult<()> {
    if self.is_empty() {
      Err(Error::empty_collection(field))
    } else {
      Ok(())
    }
  }

  fn validate_length(
    &self,
    field: &str,
    min: Option<usize>,
    max: Option<usize>,
  ) -> ValidationResult<()> {
    let len = self.len();

    if let Some(min_len) = min {
      if len < min_len {
        return Err(Error::empty_collection(field));
      }
    }

    if let Some(max_len) = max {
      if len > max_len {
        return Err(Error::too_many_items(field, max_len, len));
      }
    }

    Ok(())
  }
}

/// Extension trait for numeric validation
pub trait NumericValidateExt<T> {
  fn validate_positive(self, field: &str) -> ValidationResult<T>;
  fn validate_non_negative(self, field: &str) -> ValidationResult<T>;
  fn validate_non_zero(self, field: &str) -> ValidationResult<T>;
  fn validate_range(self, field: &str, min: T, max: T) -> ValidationResult<T>;
}

macro_rules! impl_numeric_validate {
  ($($t:ty),*) => {
    $(
      impl NumericValidateExt<$t> for $t {
        fn validate_positive(self, field: &str) -> ValidationResult<$t> {
          if self <= 0 as $t {
            Err(Error::not_positive(field, self.to_string()))
          } else {
            Ok(self)
          }
        }

        fn validate_non_negative(self, field: &str) -> ValidationResult<$t> {
          if self < 0 as $t {
            Err(Error::negative(field, self.to_string()))
          } else {
            Ok(self)
          }
        }

        fn validate_non_zero(self, field: &str) -> ValidationResult<$t> {
          if self == 0 as $t {
            Err(Error::zero(field))
          } else {
            Ok(self)
          }
        }

        fn validate_range(self, field: &str, min: $t, max: $t) -> ValidationResult<$t> {
          if self < min || self > max {
            Err(Error::numeric_range(field, min.to_string(), max.to_string(), self.to_string()))
          } else {
            Ok(self)
          }
        }
      }
    )*
  };
}

impl_numeric_validate!(
  i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

/// Macro for easy validation building
#[macro_export]
macro_rules! validate {
  ($($validation:expr),* $(,)?) => {{
    let mut builder = $crate::ValidationBuilder::new();
    $(
      builder.validate($validation);
    )*
    builder.finish()
  }};
}

/// Macro for conditional validation
#[macro_export]
macro_rules! validate_if {
  ($condition:expr => $error:expr) => {
    if $condition { Err($error) } else { Ok(()) }
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_classification() {
    let missing_err = Error::missing_field("username");
    assert!(missing_err.is_missing());
    assert!(!missing_err.is_format_error());

    let format_err = Error::invalid_email("not-an-email");
    assert!(format_err.is_format_error());
    assert!(!format_err.is_missing());

    let size_err = Error::too_long("password", 10, 15);
    assert!(size_err.is_size_error());
    assert_eq!(size_err.field_name(), Some("password"));
  }

  #[test]
  fn test_validation_builder() {
    let mut builder = ValidationBuilder::new();
    builder
      .error_if(true, || Error::missing_field("username"))
      .error_if(false, || Error::missing_field("password"))
      .validate("".validate_not_empty("email"));

    assert_eq!(builder.error_count(), 2);
    let result = builder.finish();
    assert!(result.is_err());
  }

  #[test]
  fn test_numeric_validation() {
    assert!(5i32.validate_positive("count").is_ok());
    assert!((-1i32).validate_positive("count").is_err());
    assert!(0u32.validate_non_negative("index").is_ok());
    assert!(10.5f64.validate_range("percentage", 0.0, 100.0).is_ok());
  }

  #[test]
  fn test_validation_macros() {
    let result = validate!(
      "username".validate_not_empty("username"),
      5i32.validate_positive("count"),
      "test@example.com".validate_length("email", Some(5), Some(50))
    );
    assert!(result.is_ok());

    let conditional = validate_if!(true => Error::missing_field("required"));
    assert!(conditional.is_err());
  }

  #[test]
  fn test_multiple_errors() {
    let errors = vec![
      Error::missing_field("username"),
      Error::empty_field("email"),
      Error::too_short("password", 8, 3),
    ];

    let combined = Error::combine(errors);
    if let Error::Multiple { errors } = combined {
      assert_eq!(errors.len(), 3);
    } else {
      panic!("Expected Multiple error variant");
    }
  }
}
