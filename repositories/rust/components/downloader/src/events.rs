//! Event system for download notifications and monitoring
//!
//! This module provides a comprehensive event system that allows users
//! to receive notifications about download lifecycle events, errors,
//! and status changes.

use async_trait::async_trait;
use std::fmt::Debug;

/// Events emitted during the download process.
///
/// These events provide detailed information about the download lifecycle,
/// allowing users to implement custom logging, UI updates, or other
/// response mechanisms.
#[derive(Debug, Clone)]
pub enum DownloadEvent {
  /// Preview generation has started
  PreviewStarted { url_count: usize },

  /// Preview generation completed
  PreviewCompleted {
    file_count: usize,
    total_size: Option<u64>,
    conflicts: usize
  },

  /// Download process has started
  DownloadStarted { url_count: usize },

  /// A single file download has started
  FileStarted {
    index: usize,
    url: String,
    filename: String
  },

  /// Progress update for a single file
  FileProgress {
    index: usize,
    bytes_downloaded: u64,
    total_bytes: Option<u64>,
    percentage: f64
  },

  /// A single file download completed successfully
  FileCompleted {
    index: usize,
    filename: String,
    bytes_downloaded: u64,
    duration: std::time::Duration
  },

  /// A single file download failed
  FileFailed {
    index: usize,
    url: String,
    error: String,
    retry_count: usize
  },

  /// A file download is being retried
  FileRetrying {
    index: usize,
    url: String,
    attempt: usize,
    max_attempts: usize,
    delay: std::time::Duration
  },

  /// All downloads have completed
  DownloadCompleted {
    successful: usize,
    failed: usize,
    errors: Vec<String>
  },

  /// A warning occurred (non-fatal)
  Warning {
    message: String,
    context: Option<String>
  },

  /// General information message
  Info { message: String }
}

/// Trait for handling download events.
///
/// Implement this trait to create custom event handlers that can
/// respond to download events in application-specific ways.
#[async_trait]
pub trait EventSink: Send + Sync + Debug {
  /// Called when a download event occurs.
  async fn on_event(&self, event: DownloadEvent);
}

/// A no-op event sink that discards all events.
///
/// This is useful as a default when no event handling is needed.
#[derive(Debug)]
pub struct NoOpEventSink;

#[async_trait]
impl EventSink for NoOpEventSink {
  async fn on_event(&self, _event: DownloadEvent) {
    // Do nothing
  }
}

/// An event sink that logs events using the `tracing` crate.
///
/// This provides structured logging of download events at appropriate
/// log levels based on event severity.
#[derive(Debug)]
pub struct LoggingEventSink {
  /// Whether to log progress events (can be verbose)
  pub log_progress: bool
}

impl LoggingEventSink {
  /// Creates a new logging event sink.
  pub fn new() -> Self {
    Self {
      log_progress: false
    }
  }

  /// Creates a logging event sink that includes progress events.
  pub fn with_progress() -> Self {
    Self { log_progress: true }
  }
}

impl Default for LoggingEventSink {
  fn default() -> Self {
    Self::new()
  }
}

#[async_trait]
impl EventSink for LoggingEventSink {
  async fn on_event(&self, event: DownloadEvent) {
    match event {
      DownloadEvent::PreviewStarted { url_count } => {
        info!("Starting preview generation for {} URLs", url_count);
      }

      DownloadEvent::PreviewCompleted {
        file_count,
        total_size,
        conflicts
      } => {
        if conflicts > 0 {
          warn!(
            "Preview completed: {} files, {} conflicts detected",
            file_count, conflicts
          );
        } else {
          info!("Preview completed: {} files ready for download", file_count);
        }

        if let Some(size) = total_size {
          info!("Total download size: {}", format_bytes(size));
        }
      }

      DownloadEvent::DownloadStarted { url_count } => {
        info!("Starting download of {} files", url_count);
      }

      DownloadEvent::FileStarted {
        index,
        url,
        filename
      } => {
        debug!("File {}: Starting download {} -> {}", index, url, filename);
      }

      DownloadEvent::FileProgress {
        index,
        bytes_downloaded,
        total_bytes,
        percentage
      } =>
        if self.log_progress {
          if let Some(total) = total_bytes {
            debug!(
              "File {}: {}/{} bytes ({:.1}%)",
              index, bytes_downloaded, total, percentage
            );
          } else {
            debug!("File {}: {} bytes downloaded", index, bytes_downloaded);
          }
        },

      DownloadEvent::FileCompleted {
        index,
        filename,
        bytes_downloaded,
        duration
      } => {
        info!(
          "File {}: Completed {} ({} in {:.1}s)",
          index,
          filename,
          format_bytes(bytes_downloaded),
          duration.as_secs_f64()
        );
      }

      DownloadEvent::FileFailed {
        index,
        url,
        error,
        retry_count
      } =>
        if retry_count > 0 {
          warn!(
            "File {}: Failed after {} retries - {} ({})",
            index, retry_count, url, error
          );
        } else {
          error!("File {}: Failed - {} ({})", index, url, error);
        },

      DownloadEvent::FileRetrying {
        index,
        url,
        attempt,
        max_attempts,
        delay
      } => {
        warn!(
          "File {}: Retrying {}/{} after {:.1}s - {}",
          index,
          attempt,
          max_attempts,
          delay.as_secs_f64(),
          url
        );
      }

      DownloadEvent::DownloadCompleted {
        successful,
        failed,
        errors
      } =>
        if failed == 0 {
          info!("Download completed successfully: {} files", successful);
        } else {
          warn!(
            "Download completed: {} successful, {} failed",
            successful, failed
          );
          for error in errors {
            error!("Download error: {}", error);
          }
        },

      DownloadEvent::Warning { message, context } => {
        if let Some(ctx) = context {
          warn!("{}: {}", ctx, message);
        } else {
          warn!("{}", message);
        }
      }

      DownloadEvent::Info { message } => {
        info!("{}", message);
      }
    }
  }
}

/// An event sink that collects events in memory for testing or analysis.
///
/// This is primarily useful for testing and debugging scenarios where
/// you need to inspect the sequence of events that occurred.
#[derive(Debug, Default)]
pub struct CollectingEventSink {
  events: std::sync::Mutex<Vec<DownloadEvent>>
}

impl CollectingEventSink {
  /// Creates a new collecting event sink.
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns a copy of all collected events.
  pub fn events(&self) -> Vec<DownloadEvent> {
    self.events.lock().unwrap().clone()
  }

  /// Returns the number of events collected.
  pub fn event_count(&self) -> usize {
    self.events.lock().unwrap().len()
  }

  /// Clears all collected events.
  pub fn clear(&self) {
    self.events.lock().unwrap().clear();
  }

  /// Returns events of a specific type.
  pub fn events_of_type<F>(&self, predicate: F) -> Vec<DownloadEvent>
  where
    F: Fn(&DownloadEvent) -> bool
  {
    self.events().into_iter().filter(|e| predicate(e)).collect()
  }
}

#[async_trait]
impl EventSink for CollectingEventSink {
  async fn on_event(&self, event: DownloadEvent) {
    self.events.lock().unwrap().push(event);
  }
}

/// A composite event sink that forwards events to multiple sinks.
///
/// This allows you to combine multiple event handling strategies,
/// such as logging and collecting events simultaneously.
#[derive(Debug)]
pub struct CompositeEventSink {
  sinks: Vec<Box<dyn EventSink>>
}

impl CompositeEventSink {
  /// Creates a new composite event sink.
  pub fn new() -> Self {
    Self { sinks: Vec::new() }
  }

  /// Adds an event sink to the composite.
  pub fn add_sink(mut self, sink: Box<dyn EventSink>) -> Self {
    self.sinks.push(sink);
    self
  }

  /// Convenience method to add a logging sink.
  pub fn with_logging(self) -> Self {
    self.add_sink(Box::new(LoggingEventSink::new()))
  }

  /// Convenience method to add a collecting sink.
  pub fn with_collecting(self) -> Self {
    self.add_sink(Box::new(CollectingEventSink::new()))
  }
}

impl Default for CompositeEventSink {
  fn default() -> Self {
    Self::new()
  }
}

#[async_trait]
impl EventSink for CompositeEventSink {
  async fn on_event(&self, event: DownloadEvent) {
    for sink in &self.sinks {
      sink.on_event(event.clone()).await;
    }
  }
}

/// Formats bytes as a human-readable string.
fn format_bytes(bytes: u64) -> String {
  const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

  if bytes == 0 {
    return "0 B".to_string();
  }

  let mut size = bytes as f64;
  let mut unit_index = 0;

  while size >= 1024.0 && unit_index < UNITS.len() - 1 {
    size /= 1024.0;
    unit_index += 1;
  }

  if unit_index == 0 {
    format!("{} {}", bytes, UNITS[unit_index])
  } else {
    format!("{:.1} {}", size, UNITS[unit_index])
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use std::sync::Arc;

//   #[tokio::test]
//   async fn test_collecting_event_sink() {
//     let sink = CollectingEventSink::new();

//     sink
//       .on_event(DownloadEvent::DownloadStarted { url_count: 5 })
//       .await;
//     sink
//       .on_event(DownloadEvent::FileStarted {
//         index: 0,
//         url: "http://example.com".to_string(),
//         filename: "test.txt".to_string()
//       })
//       .await;

//     assert_eq!(sink.event_count(), 2);

//     let events = sink.events();
//     assert!(matches!(events[0], DownloadEvent::DownloadStarted { .. }));
//     assert!(matches!(events[1], DownloadEvent::FileStarted { .. }));

//     sink.clear();
//     assert_eq!(sink.event_count(), 0);
//   }

//   #[tokio::test]
//   async fn test_composite_event_sink() {
//     let collector = Arc::new(CollectingEventSink::new());
//     let composite = CompositeEventSink::new()
//       .add_sink(Box::new(NoOpEventSink))
//       .add_sink(collector.clone() as Box<dyn EventSink>);

//     composite
//       .on_event(DownloadEvent::Info {
//         message: "Test message".to_string()
//       })
//       .await;

//     assert_eq!(collector.event_count(), 1);
//   }

//   #[test]
//   fn test_format_bytes() {
//     assert_eq!(format_bytes(0), "0 B");
//     assert_eq!(format_bytes(512), "512 B");
//     assert_eq!(format_bytes(1024), "1.0 KB");
//     assert_eq!(format_bytes(1536), "1.5 KB");
//     assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
//   }
// }
