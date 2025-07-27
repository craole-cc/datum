//! Progress reporting and monitoring for downloads
//!
//! This module provides real-time progress tracking capabilities,
//! allowing users to monitor download progress, estimate completion times,
//! and receive detailed statistics about ongoing downloads.

use std::{
  sync::{
    Arc,
    atomic::{AtomicUsize, Ordering}
  },
  time::{Duration, Instant}
};
use tokio::sync::{broadcast, mpsc};

#[derive(Debug, Clone)]
pub struct Reporter {
  state: Arc<State>,
  tx: broadcast::Sender<Snapshot>
}

/// Internal state for tracking progress across all downloads.
#[derive(Debug)]
struct State {
  total_files: usize,
  completed_files: AtomicUsize,
  failed_files: AtomicUsize,
  total_bytes: AtomicUsize,
  downloaded_bytes: AtomicUsize,
  start_time: Instant
}

/// A snapshot of current download progress.
///
/// This struct provides a point-in-time view of download statistics
/// and includes helpful methods for calculating percentages and rates.
#[derive(Debug, Clone, PartialEq)]
pub struct Snapshot {
  /// Total number of files to download
  pub total: usize,

  /// Number of files completed successfully
  pub completed: usize,

  /// Number of files that failed to download
  pub failed: usize,

  /// Total bytes across all files (if known)
  pub total_bytes: Option<usize>,

  /// Bytes downloaded so far
  pub downloaded_bytes: usize,

  /// Elapsed time since download started
  pub elapsed: Duration,

  /// Current download speed in bytes per second
  pub speed_bps: f64,

  /// Estimated time remaining (if calculable)
  pub eta: Option<Duration>
}

/// Progress update for a single file download.
#[derive(Debug, Clone)]
pub struct FileProgress {
  /// Index of the file being downloaded
  pub file_index: usize,

  /// Bytes downloaded for this file
  pub bytes_downloaded: usize,

  /// Total size of this file (if known)
  pub total_bytes: Option<usize>,

  /// Whether this file download is complete
  pub completed: bool,

  /// Error message if download failed
  pub error: Option<String>
}

/// Sender for progress updates from individual download tasks.
#[derive(Debug, Clone)]
pub struct Sender {
  tx: mpsc::UnboundedSender<FileProgress>
}

impl Reporter {
  /// Creates a new progress reporter for the specified number of files.
  pub fn new(total_files: usize) -> Self {
    let (tx, rx) = broadcast::channel(1024);

    let state = Arc::new(State {
      total_files,
      completed_files: AtomicUsize::new(0),
      failed_files: AtomicUsize::new(0),
      total_bytes: AtomicUsize::new(0),
      downloaded_bytes: AtomicUsize::new(0),
      start_time: Instant::now()
    });

    let reporter = Self {
      state: state.clone(),
      tx: tx.clone()
    };

    // Start progress tracking task
    let (progress_tx, mut progress_rx) = mpsc::unbounded_channel();
    let progress_sender = Sender { tx: progress_tx };

    tokio::spawn(async move {
      let mut last_update = Instant::now();
      let update_interval = Duration::from_millis(500);

      while let Some(file_progress) = progress_rx.recv().await {
        trace!("Received file progress: {:?}", file_progress);

        // Update state based on file progress
        if file_progress.completed {
          if file_progress.error.is_some() {
            state.failed_files.fetch_add(1, Ordering::Relaxed);
          } else {
            state.completed_files.fetch_add(1, Ordering::Relaxed);
          }
        }

        // Update byte counts
        if file_progress.bytes_downloaded > 0 {
          state
            .downloaded_bytes
            .fetch_add(file_progress.bytes_downloaded, Ordering::Relaxed);
        }

        if let Some(total) = file_progress.total_bytes {
          state.total_bytes.fetch_add(total, Ordering::Relaxed);
        }

        // Send snapshot if enough time has passed
        let now = Instant::now();
        if now.duration_since(last_update) >= update_interval {
          let snapshot = Self::create_snapshot(&state);
          let _ = tx.send(snapshot);
          last_update = now;
        }

        // Check if all downloads are complete
        let completed = state.completed_files.load(Ordering::Relaxed);
        let failed = state.failed_files.load(Ordering::Relaxed);
        if completed + failed >= state.total_files {
          // Send final snapshot
          let final_snapshot = Self::create_snapshot(&state);
          let _ = tx.send(final_snapshot);
          break;
        }
      }

      debug!("Progress tracking task completed");
    });

    reporter
  }

  /// Creates a progress reporter that's already completed (for edge cases).
  pub fn completed() -> Self {
    let (tx, _) = broadcast::channel(1);
    let state = Arc::new(State {
      total_files: 0,
      completed_files: AtomicUsize::new(0),
      failed_files: AtomicUsize::new(0),
      total_bytes: AtomicUsize::new(0),
      downloaded_bytes: AtomicUsize::new(0),
      start_time: Instant::now()
    });

    Self { state, tx }
  }

  /// Returns a sender for reporting progress from download tasks.
  pub fn sender(&self) -> Sender {
    // This would need to be connected to the internal progress tracking
    // For now, return a dummy sender
    let (tx, _) = mpsc::unbounded_channel();
    Sender { tx }
  }

  /// Subscribes to progress updates.
  ///
  /// Returns a receiver that will get `progress::Snapshot` updates
  /// as downloads progress.
  pub fn subscribe(&self) -> broadcast::Receiver<Snapshot> {
    self.tx.subscribe()
  }

  /// Gets the current progress snapshot.
  pub fn current_snapshot(&self) -> Snapshot {
    Self::create_snapshot(&self.state)
  }

  /// Creates a progress snapshot from the current state.
  fn create_snapshot(state: &State) -> Snapshot {
    let completed = state.completed_files.load(Ordering::Relaxed);
    let failed = state.failed_files.load(Ordering::Relaxed);
    let total_bytes_val = state.total_bytes.load(Ordering::Relaxed);
    let downloaded_bytes = state.downloaded_bytes.load(Ordering::Relaxed);
    let elapsed = state.start_time.elapsed();

    let total_bytes = if total_bytes_val > 0 {
      Some(total_bytes_val)
    } else {
      None
    };

    // Calculate download speed
    let speed_bps = if elapsed.as_secs_f64() > 0.0 {
      downloaded_bytes as f64 / elapsed.as_secs_f64()
    } else {
      0.0
    };

    // Calculate ETA
    let eta = if let Some(total) = total_bytes {
      if speed_bps > 0.0 && downloaded_bytes < total {
        let remaining_bytes = total - downloaded_bytes;
        let eta_seconds = remaining_bytes as f64 / speed_bps;
        Some(Duration::from_secs_f64(eta_seconds))
      } else {
        None
      }
    } else {
      None
    };

    Snapshot {
      total: state.total_files,
      completed,
      failed,
      total_bytes,
      downloaded_bytes,
      elapsed,
      speed_bps,
      eta
    }
  }
}

impl Snapshot {
  /// Calculates the completion percentage (0-100).
  pub fn percentage(&self) -> f64 {
    if self.total == 0 {
      return 100.0;
    }

    (self.completed as f64 / self.total as f64) * 100.0
  }

  /// Calculates the byte completion percentage (0-100) if total bytes are
  /// known.
  pub fn byte_percentage(&self) -> Option<f64> {
    self.total_bytes.map(|total| {
      if total == 0 {
        100.0
      } else {
        (self.downloaded_bytes as f64 / total as f64) * 100.0
      }
    })
  }

  /// Returns true if all downloads are complete (successful or failed).
  pub fn is_complete(&self) -> bool {
    self.completed + self.failed >= self.total
  }

  /// Returns true if all downloads completed successfully.
  pub fn is_successful(&self) -> bool {
    self.completed == self.total && self.failed == 0
  }

  /// Returns the current download speed formatted as a human-readable string.
  pub fn speed_human(&self) -> String {
    format_bytes_per_second(self.speed_bps)
  }

  /// Returns the downloaded bytes formatted as a human-readable string.
  pub fn downloaded_human(&self) -> String {
    format_bytes(self.downloaded_bytes as u64)
  }

  /// Returns the total bytes formatted as a human-readable string.
  pub fn total_bytes_human(&self) -> Option<String> {
    self.total_bytes.map(|bytes| format_bytes(bytes as u64))
  }

  /// Returns the ETA formatted as a human-readable string.
  pub fn eta_human(&self) -> Option<String> {
    self.eta.map(format_duration)
  }

  /// Returns a summary string of the current progress.
  pub fn summary(&self) -> String {
    let files_info = format!("{}/{} files", self.completed, self.total);
    let percentage = format!("{:.1}%", self.percentage());

    if let Some(total_bytes) = self.total_bytes {
      let bytes_info = format!(
        "{}/{}",
        format_bytes(self.downloaded_bytes as u64),
        format_bytes(total_bytes as u64)
      );
      let speed = self.speed_human();

      if let Some(eta) = self.eta_human() {
        format!(
          "{files_info} ({percentage}), {bytes_info} @ {speed} - ETA: {eta}"
        )
      } else {
        format!("{files_info} ({percentage}), {bytes_info} @ {speed}")
      }
    } else {
      format!("{files_info} ({percentage})")
    }
  }
}

impl Sender {
  /// Reports progress for a file download.
  pub fn report(&self, progress: FileProgress) {
    if let Err(e) = self.tx.send(progress) {
      debug!("Failed to send progress update: {}", e);
    }
  }

  /// Reports that a file download completed successfully.
  pub fn completed(&self, file_index: usize, bytes_downloaded: usize) {
    self.report(FileProgress {
      file_index,
      bytes_downloaded,
      total_bytes: None,
      completed: true,
      error: None
    });
  }

  /// Reports that a file download failed.
  pub fn failed(&self, file_index: usize, error: String) {
    self.report(FileProgress {
      file_index,
      bytes_downloaded: 0,
      total_bytes: None,
      completed: true,
      error: Some(error)
    });
  }

  /// Reports incremental progress for a file download.
  pub fn progress(
    &self,
    file_index: usize,
    bytes_downloaded: usize,
    total_bytes: Option<usize>
  ) {
    self.report(FileProgress {
      file_index,
      bytes_downloaded,
      total_bytes,
      completed: false,
      error: None
    });
  }
}

/// Formats bytes as a human-readable string (e.g., "1.5 MB").
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

/// Formats bytes per second as a human-readable string (e.g., "1.5 MB/s").
fn format_bytes_per_second(bps: f64) -> String {
  if bps < 1.0 {
    return "0 B/s".to_string();
  }

  format!("{}/s", format_bytes(bps as u64))
}

/// Formats a duration as a human-readable string (e.g., "2m 30s").
fn format_duration(duration: Duration) -> String {
  let total_seconds = duration.as_secs();

  if total_seconds < 60 {
    format!("{total_seconds}s")
  } else if total_seconds < 3600 {
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    if seconds == 0 {
      format!("{minutes}m")
    } else {
      format!("{minutes}m {seconds}s")
    }
  } else {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if minutes == 0 && seconds == 0 {
      format!("{hours}h")
    } else if seconds == 0 {
      format!("{hours}h {minutes}m")
    } else {
      format!("{hours}h {minutes}m {seconds}s")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_format_bytes() {
    assert_eq!(format_bytes(0), "0 B");
    assert_eq!(format_bytes(512), "512 B");
    assert_eq!(format_bytes(1024), "1.0 KB");
    assert_eq!(format_bytes(1536), "1.5 KB");
    assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
    assert_eq!(format_bytes(1024 * 1024 * 1024), "1.0 GB");
  }

  #[test]
  fn test_format_duration() {
    assert_eq!(format_duration(Duration::from_secs(30)), "30s");
    assert_eq!(format_duration(Duration::from_secs(60)), "1m");
    assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
    assert_eq!(format_duration(Duration::from_secs(3600)), "1h");
    assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
  }

  #[test]
  fn test_progress_snapshot_calculations() {
    let snapshot = Snapshot {
      total: 10,
      completed: 3,
      failed: 1,
      total_bytes: Some(1000),
      downloaded_bytes: 300,
      elapsed: Duration::from_secs(10),
      speed_bps: 30.0,
      eta: Some(Duration::from_secs(23))
    };

    assert_eq!(snapshot.percentage(), 30.0);
    assert_eq!(snapshot.byte_percentage(), Some(30.0));
    assert!(!snapshot.is_complete());
    assert!(!snapshot.is_successful());
  }

  #[tokio::test]
  async fn test_progress_reporter() {
    let reporter = Reporter::new(2);
    let mut rx = reporter.subscribe();

    let sender = reporter.sender();

    // Send some progress updates
    sender.completed(0, 100);
    sender.failed(1, "Network error".to_string());

    // Should receive progress updates
    let snapshot = rx.recv().await.unwrap();
    assert!(snapshot.completed > 0 || snapshot.failed > 0);
  }
}
