//! Configuration types for the downloader
//!
//! This module provides comprehensive configuration options for customizing
//! downloader behavior, including timeouts, retry policies, file handling,
//! and progress reporting.

use crate::*;
use std::{sync::Arc, time::Duration};

/// Comprehensive configuration for the downloader.
///
/// This struct provides fine-grained control over all aspects of the download
/// process, from network settings to file handling policies.
///
/// # Examples
///
/// ```rust
/// use downloader::{Config, OverwritePolicy, filename::Strategy};
/// use std::time::Duration;
///
/// let config = Config::builder()
///     .concurrency_limit(10)
///     .timeout(Duration::from_secs(60))
///     .max_retries(3)
///     .overwrite_policy(OverwritePolicy::Skip)
///     .filename_strategy(filename::Strategy::Smart)
///     .max_file_size(500 * 1024 * 1024) // 500MB
///     .user_agent("MyApp/1.0")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Config {
  /// Maximum number of concurrent downloads (None = unlimited)
  pub concurrency_limit: Option<usize>,

  /// Timeout for individual HTTP requests
  pub timeout: Duration,

  /// Maximum number of retry attempts for failed downloads
  pub max_retries: usize,

  /// Delay between retry attempts
  pub retry_delay: Duration,

  /// Policy for handling existing files
  pub overwrite_policy: OverwritePolicy,

  /// Strategy for generating filenames from URLs
  pub filename_strategy: filename::Strategy,

  /// Maximum allowed file size (None = unlimited)
  pub max_file_size: Option<u64>,

  /// Custom User-Agent header
  pub user_agent: Option<String>,

  /// Maximum number of HTTP redirects to follow
  pub max_redirects: usize,

  /// Whether to fetch file metadata during preview
  pub fetch_metadata: bool,

  /// Minimum interval between progress updates
  pub progress_interval: Duration,

  /// Event sink for notifications and progress reporting
  pub event_sink: Arc<dyn EventSink>,

  /// Whether to verify SSL certificates
  pub verify_ssl: bool,

  /// Additional HTTP headers to send with requests
  pub custom_headers: Vec<(String, String)>
}

impl Default for Config {
  fn default() -> Self {
    Self {
      concurrency_limit: Some(5),
      timeout: Duration::from_secs(30),
      max_retries: 3,
      retry_delay: Duration::from_secs(1),
      overwrite_policy: OverwritePolicy::Error,
      filename_strategy: filename::Strategy::Smart,
      max_file_size: None,
      user_agent: Some(format!(
        "{}/{} ({})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY")
      )),
      max_redirects: 10,
      fetch_metadata: true,
      progress_interval: Duration::from_millis(500),
      event_sink: Arc::new(events::NoOpEventSink),
      verify_ssl: true,
      custom_headers: Vec::new()
    }
  }
}

impl Config {
  /// Creates a new configuration builder.
  pub fn builder() -> ConfigBuilder {
    ConfigBuilder::new()
  }

  /// Creates a configuration optimized for fast downloads.
  pub fn fast() -> Self {
    Self {
      concurrency_limit: Some(10),
      timeout: Duration::from_secs(15),
      max_retries: 1,
      retry_delay: Duration::from_millis(500),
      fetch_metadata: false,
      progress_interval: Duration::from_millis(100),
      ..Default::default()
    }
  }

  /// Creates a configuration optimized for reliable downloads.
  pub fn reliable() -> Self {
    Self {
      concurrency_limit: Some(3),
      timeout: Duration::from_secs(120),
      max_retries: 5,
      retry_delay: Duration::from_secs(2),
      fetch_metadata: true,
      progress_interval: Duration::from_secs(1),
      ..Default::default()
    }
  }

  /// Creates a configuration for downloading large files.
  pub fn large_files() -> Self {
    Self {
      concurrency_limit: Some(2),
      timeout: Duration::from_secs(300),
      max_retries: 3,
      retry_delay: Duration::from_secs(5),
      max_file_size: None,
      fetch_metadata: true,
      progress_interval: Duration::from_millis(250),
      ..Default::default()
    }
  }
}

/// Builder for creating `Config` instances.
///
/// This builder provides a fluent interface for constructing configuration
/// objects with only the settings you want to customize.
#[derive(Debug)]
pub struct ConfigBuilder {
  config: Config
}

impl ConfigBuilder {
  /// Creates a new builder with default settings.
  pub fn new() -> Self {
    Self {
      config: Config::default()
    }
  }

  /// Sets the concurrency limit.
  pub fn concurrency_limit(mut self, limit: Option<usize>) -> Self {
    self.config.concurrency_limit = limit;
    self
  }

  /// Sets the request timeout.
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.config.timeout = timeout;
    self
  }

  /// Sets the maximum number of retry attempts.
  pub fn max_retries(mut self, retries: usize) -> Self {
    self.config.max_retries = retries;
    self
  }

  /// Sets the delay between retry attempts.
  pub fn retry_delay(mut self, delay: Duration) -> Self {
    self.config.retry_delay = delay;
    self
  }

  /// Sets the overwrite policy for existing files.
  pub fn overwrite_policy(mut self, policy: OverwritePolicy) -> Self {
    self.config.overwrite_policy = policy;
    self
  }

  pub fn skip_existing(mut self) -> Self {
    self.config.overwrite_policy = OverwritePolicy::Skip;
    self
  }

  pub fn overwrite_existing(mut self) -> Self {
    self.config.overwrite_policy = OverwritePolicy::Overwrite;
    self
  }

  pub fn fail_on_existing(mut self) -> Self {
    self.config.overwrite_policy = OverwritePolicy::Error;
    self
  }

  pub fn rename_exiting(mut self) -> Self {
    self.config.overwrite_policy = OverwritePolicy::Rename;
    self
  }

  /// Sets the filename extraction strategy.
  pub fn filename_strategy(mut self, strategy: filename::Strategy) -> Self {
    self.config.filename_strategy = strategy;
    self
  }

  /// Sets the maximum allowed file size.
  pub fn max_file_size(mut self, size: Option<u64>) -> Self {
    self.config.max_file_size = size;
    self
  }

  /// Sets a custom User-Agent header.
  pub fn user_agent<S: Into<String>>(mut self, agent: S) -> Self {
    self.config.user_agent = Some(agent.into());
    self
  }

  /// Sets the maximum number of redirects to follow.
  pub fn max_redirects(mut self, redirects: usize) -> Self {
    self.config.max_redirects = redirects;
    self
  }

  /// Enables or disables metadata fetching during preview.
  pub fn fetch_metadata(mut self, fetch: bool) -> Self {
    self.config.fetch_metadata = fetch;
    self
  }

  /// Sets the progress update interval.
  pub fn progress_interval(mut self, interval: Duration) -> Self {
    self.config.progress_interval = interval;
    self
  }

  /// Sets a custom event sink for notifications.
  pub fn event_sink(mut self, sink: Arc<dyn EventSink>) -> Self {
    self.config.event_sink = sink;
    self
  }

  /// Enables or disables SSL certificate verification.
  pub fn verify_ssl(mut self, verify: bool) -> Self {
    self.config.verify_ssl = verify;
    self
  }

  /// Adds a custom HTTP header.
  pub fn header<K: Into<String>, V: Into<String>>(
    mut self,
    key: K,
    value: V
  ) -> Self {
    self.config.custom_headers.push((key.into(), value.into()));
    self
  }

  /// Adds multiple custom HTTP headers.
  pub fn headers<I, K, V>(mut self, headers: I) -> Self
  where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<String>
  {
    for (key, value) in headers {
      self.config.custom_headers.push((key.into(), value.into()));
    }
    self
  }

  /// Builds the final configuration.
  pub fn build(self) -> Config {
    self.config
  }
}

impl Default for ConfigBuilder {
  fn default() -> Self {
    Self::new()
  }
}

/// Policy for handling files that already exist at the target location.
#[derive(Debug, Clone, PartialEq)]
pub enum OverwritePolicy {
  /// Skip downloading files that already exist
  Skip,

  /// Overwrite existing files without warning
  Overwrite,

  /// Return an error if any target file already exists
  Error,

  /// Automatically rename new files to avoid conflicts
  Rename
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.concurrency_limit, Some(5));
    assert_eq!(config.timeout, Duration::from_secs(30));
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.overwrite_policy, OverwritePolicy::Error);
  }

  #[test]
  fn test_builder_pattern() {
    let config = Config::builder()
      .concurrency_limit(Some(10))
      .timeout(Duration::from_secs(60))
      .max_retries(5)
      .overwrite_policy(OverwritePolicy::Skip)
      .user_agent("Test/1.0")
      .header("Authorization", "Bearer token")
      .build();

    assert_eq!(config.concurrency_limit, Some(10));
    assert_eq!(config.timeout, Duration::from_secs(60));
    assert_eq!(config.max_retries, 5);
    assert_eq!(config.overwrite_policy, OverwritePolicy::Skip);
    assert_eq!(config.user_agent, Some("Test/1.0".to_string()));
    assert_eq!(config.custom_headers.len(), 1);
  }

  #[test]
  fn test_preset_configs() {
    let fast = Config::fast();
    assert_eq!(fast.concurrency_limit, Some(10));
    assert_eq!(fast.max_retries, 1);
    assert!(!fast.fetch_metadata);

    let reliable = Config::reliable();
    assert_eq!(reliable.concurrency_limit, Some(3));
    assert_eq!(reliable.max_retries, 5);
    assert!(reliable.fetch_metadata);

    let large = Config::large_files();
    assert_eq!(large.concurrency_limit, Some(2));
    assert_eq!(large.timeout, Duration::from_secs(300));
    assert_eq!(large.max_file_size, None);
  }
}
