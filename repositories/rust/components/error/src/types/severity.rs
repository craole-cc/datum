/// Error severity levels for better error handling and logging
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
  /// Low severity - warning level, operation might continue
  Low,
  /// Medium severity - error that affects functionality but might be recoverable
  Medium,
  /// High severity - critical error that stops processing
  High,
  /// Critical severity - system-level error requiring immediate attention
  Critical,
}

impl std::fmt::Display for Severity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Low => write!(f, "Low"),
      Self::Medium => write!(f, "Medium"),
      Self::High => write!(f, "High"),
      Self::Critical => write!(f, "Critical"),
    }
  }
}
