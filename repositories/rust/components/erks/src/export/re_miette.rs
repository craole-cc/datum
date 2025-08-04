//! Enhanced error handling utilities for miette
//!
//! This module provides macros and traits for creating rich error traces
//! that show the full call path from error origin to caller.
pub use miette::{
  Context, Diagnostic, IntoDiagnostic, LabeledSpan, MietteHandlerOpts,
  NamedSource, Report, Result, Severity, Severity as MietteSeverity,
  SourceSpan, WrapErr, bail, ensure, miette, set_hook, set_panic_hook, *,
};

/// Trait for adding source location information to errors
///
/// This trait extends any error type that can be converted to a miette Report
/// with the ability to tag it with file, line, and column information.
pub trait WithLocation {
  fn with_source_location(
    self,
    file: &'static str,
    line: u32,
    column: u32,
  ) -> Report;
}

impl<E> WithLocation for E
where
  E: Into<Report>,
{
  fn with_source_location(
    self,
    file: &'static str,
    line: u32,
    column: u32,
  ) -> Report {
    self
      .into()
      .wrap_err(format!("Error originated at {file}:{line}:{column}"))
  }
}

pub trait ResultLogger<T> {
  fn log_error_and_continue(self);
  fn log_error_with_context(self, context: impl Into<String>);
  fn bail_with_context(self, context: impl Into<String>) -> Result<T>; // No logging
}

impl<T, E> ResultLogger<T> for Result<T, E>
where
  E: Into<Report>,
{
  fn log_error_and_continue(self) {
    if let Err(e) = self {
      let report = e.into();
      eprintln!("{report:?}");
    }
  }

  fn log_error_with_context(self, context: impl Into<String>) {
    if let Err(e) = self {
      let report = e.into().wrap_err(context.into());
      eprintln!("{report:?}");
    }
  }

  fn bail_with_context(self, context: impl Into<String>) -> Result<T> {
    // No logging here - let the original error chain handle it
    self.map_err(|e| e.into().wrap_err(context.into()))
  }
}

/// Tags an error with the location where error handling occurs
///
/// # Example
/// ```rust
/// let file = tag_error!(
///   File::open(path)
///     .into_diagnostic()
///     .wrap_err("Failed to open file")
/// )?;
/// ```
#[macro_export]
macro_rules! tag_error {
  ($expr:expr) => {
    $expr.map_err(|e| e.with_source_location(file!(), line!(), column!()))
  };
}

/// Adds function name and location to error trace
///
/// Use this to wrap the entire result of a function, showing which function
/// the error passed through.
///
/// # Example
/// ```rust
/// pub fn my_function() -> Result<String> {
///   let result = || -> Result<String> {
///     // function logic here
///     Ok("success".to_string())
///   };
///   trace_fn!("my_function", result())
/// }
/// ```
#[macro_export]
macro_rules! trace_fn {
  ($fn_name:expr, $expr:expr) => {
    $expr.wrap_err(format!(
      "In function '{}' at {}:{}:{}",
      $fn_name,
      file!(),
      line!(),
      column!()
    ))
  };
}

/// Adds call site information to error trace
///
/// Use this when calling functions that might return errors to show
/// where the call was made from.
///
/// # Example
/// ```rust
/// match trace_call!(some_function_that_might_fail()) {
///   Ok(result) => println!("Success: {result}"),
///   Err(e) => eprintln!("Error: {e:?}"),
/// }
/// ```
#[macro_export]
macro_rules! trace_call {
  ($expr:expr) => {
    $expr.wrap_err(format!("Called from {}:{}:{}", file!(), line!(), column!()))
  };
}

/// Adds custom context message to error with location
///
/// Simple wrapper to add a custom message while preserving the error chain.
///
/// # Example
/// ```rust
/// let data = error_msg!(
///   std::fs::read_to_string(path),
///   "Failed to read configuration file"
/// )?;
/// ```
#[macro_export]
macro_rules! error_msg {
  ($expr:expr, $msg:expr) => {
    $expr.into_diagnostic().wrap_err(format!(
      "{} (at {}:{})",
      $msg,
      file!(),
      line!()
    ))
  };
}

// /// Creates a comprehensive error with rich diagnostics, tracing, and location info
// ///
// /// This macro combines automatic error logging, custom messages, help text, error codes,
// /// severity levels, labels, and precise source location tracking. The tracing level
// /// is automatically selected based on the severity, and code/labels appear on the log line.
// ///
// /// # Features
// /// - Automatic logging with appropriate tracing level based on severity
// /// - Code and label information in the tracing output
// /// - Source location capture (file:line:column)
// /// - Rich miette diagnostics with help text, error codes, severity, and labels
// /// - Preserves original error in the chain
// ///
// /// # Output Format
// /// ```text
// /// ERROR: FILE_OPEN_ERROR
// ///   × Failed to open file for delimiter detection (at src/config.rs:42:15)
// ///
// ///       Caused by: No such file or directory (os error 2)
// ///   help: Ensure the file exists and you have read permissions
// /// ```
// #[macro_export]
// macro_rules! enriched_error {
//   // Helper macro to choose tracing level and format message
//   (@log $severity:expr, $code:expr, $label:expr) => {
//     match $severity {
//       Severity::Error => error!("{} {}", $code.on_red(), $label.red()),
//       Severity::Warning | Severity::Advice => warn!("{} {}", $code.on_yellow(), $label.yellow()),
//     }
//   };

//   (@log $severity:expr, $code:expr) => {
//     match $severity {
//       Severity::Error => error!("{}", $code.red()),
//       Severity::Warning | Severity::Advice => warn!("{}", $code.yellow()),
//     }
//   };

//   (@log $severity:expr) => {
//     match $severity {
//       Severity::Error => error!(""),
//       Severity::Warning | Severity::Advice => warn!(""),
//     }
//   };

//   // Full version with all options
//   ($expr:expr, $msg:expr, code = $code:expr, help = $help:expr, severity = $severity:expr, labels = $labels:expr) => {{
//     let location = format!("{}:{}:{}", file!(), line!(), column!());
//     // Extract first label text if available
//     let label_text = $labels.first().map(|l| l.label().unwrap_or("")).unwrap_or("");
//     enriched_error!(@log $severity, $code, label_text);
//     $expr.into_diagnostic().map_err(|original_error| {
//       miette!(
//         // code = $code,
//         help = $help,
//         severity = $severity,
//         labels = $labels,
//         "{} (at {})\n\nCaused by: {}",
//         $msg,
//         location,
//         original_error
//       )
//     })
//   }};

//   // Code, help, and severity
//   ($expr:expr, $msg:expr, code = $code:expr, help = $help:expr, severity = $severity:expr) => {{
//     let location = format!("{}:{}:{}", file!(), line!(), column!());
//     enriched_error!(@log $severity, $code);
//     $expr.into_diagnostic().map_err(|original_error| {
//       miette!(
//         // code = $code,
//         help = $help,
//         severity = $severity,
//         "{} (at {})\n\nCaused by: {}",
//         $msg,
//         location,
//         original_error
//       )
//     })
//   }};

//   // Code, help, and labels (default to Error severity)
//   ($expr:expr, $msg:expr, code = $code:expr, help = $help:expr, labels = $labels:expr) => {{
//     let location = format!("{}:{}:{}", file!(), line!(), column!());
//     let label_text = $labels.first().map(|l| l.label().unwrap_or("")).unwrap_or("");
//     error!("{} {}", $code, label_text);
//     $expr.into_diagnostic().map_err(|original_error| {
//       miette!(
//         // code = $code,
//         help = $help,
//         labels = $labels,
//         "{} (at {})\n\nCaused by: {}",
//         $msg,
//         location,
//         original_error
//       )
//     })
//   }};

//   // Help and severity
//   ($expr:expr, $msg:expr, help = $help:expr, severity = $severity:expr) => {{
//     let location = format!("{}:{}:{}", file!(), line!(), column!());
//     enriched_error!(@log $severity);
//     $expr.into_diagnostic().map_err(|original_error| {
//       miette!(
//         help = $help,
//         severity = $severity,
//         "{} (at {})\n\nCaused by: {}",
//         $msg,
//         location,
//         original_error
//       )
//     })
//   }};

//   // Help and labels (default to Error severity)
//   ($expr:expr, $msg:expr, help = $help:expr, labels = $labels:expr) => {{
//     let location = format!("{}:{}:{}", file!(), line!(), column!());
//     let label_text = $labels.first().map(|l| l.label().unwrap_or("")).unwrap_or("");
//     error!("{}", label_text);
//     $expr.into_diagnostic().map_err(|original_error| {
//       miette!(
//         help = $help,
//         labels = $labels,
//         "{} (at {})\n\nCaused by: {}",
//         $msg,
//         location,
//         original_error
//       )
//     })
//   }};

//   // Just severity
//   ($expr:expr, $msg:expr, severity = $severity:expr) => {{
//     let location = format!("{}:{}:{}", file!(), line!(), column!());
//     enriched_error!(@log $severity);
//     $expr.into_diagnostic().map_err(|original_error| {
//       miette!(
//         severity = $severity,
//         "{} (at {})\n\nCaused by: {}",
//         $msg,
//         location,
//         original_error
//       )
//     })
//   }};

//   // Just labels (default to Error severity)
//   ($expr:expr, $msg:expr, labels = $labels:expr) => {{
//     let location = format!("{}:{}:{}", file!(), line!(), column!());
//     let label_text = $labels.first().map(|l| l.label().unwrap_or("")).unwrap_or("");
//     error!("{}", label_text);
//     $expr.into_diagnostic().map_err(|original_error| {
//       miette!(
//         labels = $labels,
//         "{} (at {})\n\nCaused by: {}",
//         $msg,
//         location,
//         original_error
//       )
//     })
//   }};

//   // Just code (default to Error severity)
//   ($expr:expr, $msg:expr, code = $code:expr) => {{
//     let location = format!("{}:{}:{}", file!(), line!(), column!());
//     error!("{}", $code);
//     $expr.into_diagnostic().map_err(|original_error| {
//       miette!(
//         // code = $code,
//         "{} (at {})\n\nCaused by: {}",
//         $msg,
//         location,
//         original_error
//       )
//     })
//   }};

//   // Original variants (unchanged for backward compatibility)
//   ($expr:expr, $msg:expr, help = $help:expr) => {{
//     let location = format!("{}:{}:{}", file!(), line!(), column!());
//     error!("");
//     $expr.into_diagnostic().map_err(|original_error| {
//       miette!(
//         help = $help,
//         "{} (at {})\n\nCaused by: {}",
//         $msg,
//         location,
//         original_error
//       )
//     })
//   }};

//   ($expr:expr, $msg:expr) => {{
//     let location = format!("{}:{}:{}", file!(), line!(), column!());
//     error!("");
//     $expr.into_diagnostic().map_err(|original_error| {
//       miette!(
//         "{} (at {})\n\nCaused by: {}",
//         $msg,
//         location,
//         original_error
//       )
//     })
//   }};
// }

/// A macro for enriching errors with contextual information, logging, and miette diagnostics.
///
/// This macro wraps expressions that return `Result` types and enhances any errors with:
/// - Source location information (file, line, column)
/// - Custom error messages and help text
/// - Severity levels for appropriate logging
/// - Optional labels for source code spans
/// - Colored console output based on severity
///
/// The macro only logs when an actual error occurs, preventing spurious error messages
/// for successful operations.
///
/// # Requirements
///
/// This macro requires the following dependencies:
/// ```toml
/// [dependencies]
/// miette = { version = "5.0", features = ["fancy"] }
/// tracing = "0.1"
/// colored = "2.0"
/// ```
///
/// # Examples
///
/// ## Basic usage with help text
/// ```rust
/// use std::fs::File;
/// use miette::{IntoDiagnostic, Result, Severity};
///
/// # // Mock the macro for doctest
/// # macro_rules! enriched_error {
/// #     ($expr:expr, $msg:expr, help = $help:expr) => {
/// #         $expr.into_diagnostic().map_err(|e| miette::miette!(help = $help, "{}", $msg))
/// #     };
/// # }
///
/// fn open_config() -> Result<File> {
///     enriched_error!(
///         File::open("config.toml"),
///         "Failed to open configuration file",
///         help = "Make sure config.toml exists in the current directory"
///     )
/// }
/// ```
///
/// ## With severity and custom code
/// ```rust
/// use std::fs::File;
/// use miette::{IntoDiagnostic, Result, Severity};
///
/// # macro_rules! enriched_error {
/// #     ($expr:expr, $msg:expr, code = $code:expr, severity = $severity:expr) => {
/// #         $expr.into_diagnostic().map_err(|e| miette::miette!(severity = $severity, "{}", $msg))
/// #     };
/// # }
///
/// fn open_optional_file() -> Result<File> {
///     enriched_error!(
///         File::open("optional.txt"),
///         "Optional file not found",
///         code = format!("OPTIONAL_FILE_MISSING"),
///         severity = Severity::Warning
///     )
/// }
/// ```
///
/// ## With labels for source code spans
/// ```rust
/// use std::fs::File;
/// use miette::{IntoDiagnostic, Result, LabeledSpan, Severity};
///
/// # macro_rules! enriched_error {
/// #     ($expr:expr, $msg:expr, labels = $labels:expr) => {
/// #         $expr.into_diagnostic().map_err(|e| miette::miette!(labels = $labels, "{}", $msg))
/// #     };
/// # }
///
/// fn parse_with_context() -> Result<()> {
///     let labels = vec![LabeledSpan::at(10..20, "problematic section")];
///     enriched_error!(
///         std::str::from_utf8(&[0xFF, 0xFE]),
///         "Invalid UTF-8 sequence detected",
///         labels = labels
///     )?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! enriched_error {
    // Helper macro to choose tracing level and format message
    (@log $severity:expr, $code:expr, $label:expr) => {
        match $severity {
            Severity::Error => error!("{} {}", $code.on_red(), $label.red()),
            Severity::Warning | Severity::Advice => warn!("{} {}", $code.on_yellow(), $label.yellow()),
        }
    };

    (@log $severity:expr, $code:expr) => {
        match $severity {
            Severity::Error => error!("{}", $code.red()),
            Severity::Warning | Severity::Advice => warn!("{}", $code.yellow()),
        }
    };

    (@log $severity:expr) => {
        match $severity {
            Severity::Error => error!("Operation failed"),
            Severity::Warning | Severity::Advice => warn!("Operation failed"),
        }
    };

    // Full version with all options
    ($expr:expr, $msg:expr, code = $code:expr, help = $help:expr, severity = $severity:expr, labels = $labels:expr) => {{
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        $expr.into_diagnostic().map_err(|original_error| {
            // Only log when there's actually an error
            let label_text = $labels.first().map(|l| l.label().unwrap_or("")).unwrap_or("");
            enriched_error!(@log $severity, $code, label_text);

            miette!(
                help = $help,
                severity = $severity,
                labels = $labels,
                "{} (at {})\n\nCaused by: {}",
                $msg,
                location,
                original_error
            )
        })
    }};

    // Code, help, and severity
    ($expr:expr, $msg:expr, code = $code:expr, help = $help:expr, severity = $severity:expr) => {{
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        $expr.into_diagnostic().map_err(|original_error| {
            enriched_error!(@log $severity, $code);
            miette!(
                help = $help,
                severity = $severity,
                "{} (at {})\n\nCaused by: {}",
                $msg,
                location,
                original_error
            )
        })
    }};

    // Code, help, and labels (default to Error severity)
    ($expr:expr, $msg:expr, code = $code:expr, help = $help:expr, labels = $labels:expr) => {{
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        $expr.into_diagnostic().map_err(|original_error| {
            let label_text = $labels.first().map(|l| l.label().unwrap_or("")).unwrap_or("");
            error!("{} {}", $code, label_text);
            miette::miette!(
                help = $help,
                labels = $labels,
                "{} (at {})\n\nCaused by: {}",
                $msg,
                location,
                original_error
            )
        })
    }};

    // Help and severity
    ($expr:expr, $msg:expr, help = $help:expr, severity = $severity:expr) => {{
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        $expr.into_diagnostic().map_err(|original_error| {
            enriched_error!(@log $severity);
            miette::miette!(
                help = $help,
                severity = $severity,
                "{} (at {})\n\nCaused by: {}",
                $msg,
                location,
                original_error
            )
        })
    }};

    // Help and labels (default to Error severity)
    ($expr:expr, $msg:expr, help = $help:expr, labels = $labels:expr) => {{
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        $expr.into_diagnostic().map_err(|original_error| {
            let label_text = $labels.first().map(|l| l.label().unwrap_or("")).unwrap_or("");
            error!("{}", label_text);
            miette::miette!(
                help = $help,
                labels = $labels,
                "{} (at {})\n\nCaused by: {}",
                $msg,
                location,
                original_error
            )
        })
    }};

    // Just severity
    ($expr:expr, $msg:expr, severity = $severity:expr) => {{
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        $expr.into_diagnostic().map_err(|original_error| {
            enriched_error!(@log $severity);
            miette::miette!(
                severity = $severity,
                "{} (at {})\n\nCaused by: {}",
                $msg,
                location,
                original_error
            )
        })
    }};

    // Just labels (default to Error severity)
    ($expr:expr, $msg:expr, labels = $labels:expr) => {{
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        $expr.into_diagnostic().map_err(|original_error| {
            let label_text = $labels.first().map(|l| l.label().unwrap_or("")).unwrap_or("");
            error!("{}", label_text);
            miette::miette!(
                labels = $labels,
                "{} (at {})\n\nCaused by: {}",
                $msg,
                location,
                original_error
            )
        })
    }};

    // Just code (default to Error severity)
    ($expr:expr, $msg:expr, code = $code:expr) => {{
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        $expr.into_diagnostic().map_err(|original_error| {
            error!("{}", $code);
            miette::miette!(
                "{} (at {})\n\nCaused by: {}",
                $msg,
                location,
                original_error
            )
        })
    }};

    // Just help text
    ($expr:expr, $msg:expr, help = $help:expr) => {{
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        $expr.into_diagnostic().map_err(|original_error| {
            error!("Operation failed");
            miette::miette!(
                help = $help,
                "{} (at {})\n\nCaused by: {}",
                $msg,
                location,
                original_error
            )
        })
    }};

    // Minimal version - just message
    ($expr:expr, $msg:expr) => {{
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        $expr.into_diagnostic().map_err(|original_error| {
            error!("Operation failed");
            miette::miette!(
                "{} (at {})\n\nCaused by: {}",
                $msg,
                location,
                original_error
            )
        })
    }};
}
