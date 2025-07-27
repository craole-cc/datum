//! Core downloader functionality with enhanced user experience
//!
//! This module provides the main `Downloader` struct with comprehensive
//! configuration options, progress reporting, and user-friendly error handling.

use crate::*;
use std::{
  collections::HashMap,
  path::{Path, PathBuf},
  sync::Arc,
  time::Duration
};
use tokio::{
  fs::{create_dir_all, remove_dir_all},
  task::JoinHandle
};

#[derive(Debug)]
pub struct Downloader {
  /// URLs to download
  urls: Vec<String>,
  /// Target directory
  target_dir: PathBuf,
  /// Configuration settings
  config: Config,
  /// HTTP client with configured settings
  client: ReqwestClient,
  /// URL validator
  validator: validation::UrlValidator,
  /// Event sink for notifications
  event_sink: Arc<dyn EventSink>,
  /// Cached validation results
  validated_urls: Option<Vec<validation::Url>>
}

impl Default for Downloader {
  fn default() -> Self {
    Self {
      urls: Vec::new(),
      target_dir: PathBuf::new(),
      config: Config::default(),
      client: ReqwestClient::new(),
      validator: validation::UrlValidator::default(),
      event_sink: Arc::new(LoggingEventSink::default()),
      validated_urls: None
    }
  }
}

impl Downloader {
  pub fn new<S, P>(urls: Vec<S>, target_dir: P) -> Result<Self>
  where
    S: AsRef<str>,
    P: AsRef<Path>
  {
    Ok(Self {
      urls: urls.into_iter().map(|s| s.as_ref().to_string()).collect(),
      target_dir: target_dir.as_ref().to_path_buf(),
      ..Default::default()
    })
  }

  pub fn new_with_config<S, P>(
    urls: Vec<S>,
    target_dir: P,
    config: Config
  ) -> Result<Self>
  where
    S: AsRef<str>,
    P: AsRef<Path>
  {
    Ok(Self {
      urls: urls.into_iter().map(|s| s.as_ref().to_string()).collect(),
      target_dir: target_dir.as_ref().to_path_buf(),
      config,
      ..Default::default()
    })
  }

  pub fn with_concurrency_limit(&mut self, limit: usize) -> &mut Self {
    self.config.concurrency_limit = Some(limit);
    self
  }

  pub fn with_timeout(&mut self, timeout: Duration) -> &mut Self {
    self.config.timeout = timeout;
    self
  }

  pub fn with_max_retries(&mut self, retries: usize) -> &mut Self {
    self.config.max_retries = retries;
    self
  }

  pub fn with_retry_delay(&mut self, delay: Duration) -> &mut Self {
    self.config.retry_delay = delay;
    self
  }

  pub fn with_overwrite_policy(
    &mut self,
    policy: OverwritePolicy
  ) -> &mut Self {
    self.config.overwrite_policy = policy;
    self
  }

  pub fn skip_existing(&mut self) -> &mut Self {
    self.config.overwrite_policy = OverwritePolicy::Skip;
    self
  }

  pub fn overwrite_existing(&mut self) -> &mut Self {
    self.config.overwrite_policy = OverwritePolicy::Overwrite;
    self
  }

  pub fn fail_on_existing(&mut self) -> &mut Self {
    self.config.overwrite_policy = OverwritePolicy::Error;
    self
  }

  pub fn rename_existing(&mut self) -> &mut Self {
    self.config.overwrite_policy = OverwritePolicy::Rename;
    self
  }

  pub fn with_filename_strategy(
    &mut self,
    strategy: filename::Strategy
  ) -> &mut Self {
    self.config.filename_strategy = strategy;
    self
  }

  pub fn with_max_file_size(&mut self, size: Option<u64>) -> &mut Self {
    self.config.max_file_size = size;
    self
  }

  pub fn with_user_agent<S: Into<String>>(&mut self, agent: S) -> &mut Self {
    self.config.user_agent = Some(agent.into());
    self
  }

  pub fn with_max_redirects(&mut self, redirects: usize) -> &mut Self {
    self.config.max_redirects = redirects;
    self
  }

  pub fn with_fetch_metadata(&mut self, fetch: bool) -> &mut Self {
    self.config.fetch_metadata = fetch;
    self
  }

  pub fn with_progress_interval(&mut self, interval: Duration) -> &mut Self {
    self.config.progress_interval = interval;
    self
  }

  pub fn with_event_sink(&mut self, sink: Arc<dyn EventSink>) -> &mut Self {
    self.config.event_sink = sink;
    self
  }

  pub fn with_ssl_verification(&mut self, verify: bool) -> &mut Self {
    self.config.verify_ssl = verify;
    self
  }

  pub fn with_header<K: Into<String>, V: Into<String>>(
    &mut self,
    key: K,
    value: V
  ) -> &mut Self {
    self.config.custom_headers.push((key.into(), value.into()));
    self
  }

  pub fn with_headers<I, K, V>(&mut self, headers: I) -> &mut Self
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

  /// Validates all URLs and generates a preview of what will be downloaded.
  ///
  /// This method performs URL validation, filename extraction, conflict
  /// detection, and optionally fetches metadata like file sizes. It's useful
  /// for showing users what will happen before starting the actual downloads.
  ///
  /// # Returns
  ///
  /// Returns a `preview::Manifest` containing detailed information about each
  /// file, potential conflicts, and warnings.
  pub async fn preview(&mut self) -> Result<preview::Manifest> {
    trace!("Generating download preview for {} URLs", self.urls.len());

    self
      .event_sink
      .on_event(DownloadEvent::PreviewStarted {
        url_count: self.urls.len()
      })
      .await;

    let validated_urls = self.validate_urls().await?;
    self.validated_urls = Some(validated_urls.clone());

    let mut files = Vec::new();
    let mut total_size = Some(0u64);
    let mut conflicts = HashMap::<String, Vec<String>>::new();
    let mut warnings = Vec::new();

    // Process each validated URL
    for validated in &validated_urls {
      let estimated_size = if self.config.fetch_metadata {
        utils::fetch_content_length(&self.client, &validated.parsed).await
      } else {
        None
      };

      // Update total size calculation
      if let (Some(total), Some(size)) = (total_size.as_mut(), estimated_size) {
        *total += size;
      } else if estimated_size.is_some() {
        total_size = None; // Can't calculate total if any size is unknown
      }

      let status = preview::Status::new(
        self.config.max_file_size,
        validated,
        estimated_size
      )
      .await;

      // Check for filename conflicts
      conflicts
        .entry(validated.filename.clone())
        .or_default()
        .push(validated.original.clone());

      files.push(preview::Target {
        url: validated.original.clone(),
        filename: validated.filename.clone(),
        target_path: validated.target_path.clone(),
        estimated_size,
        status
      });
    }

    // Convert conflicts map to conflict list
    let conflicts: Vec<preview::Conflict> = conflicts
      .into_iter()
      .filter(|(_, urls)| urls.len() > 1)
      .map(|(filename, urls)| preview::Conflict { filename, urls })
      .collect();

    // Generate warnings
    if !conflicts.is_empty() {
      warnings.push(format!("{} filename conflicts detected", conflicts.len()));
    }

    let existing_count = files
      .iter()
      .filter(|f| matches!(f.status, preview::Status::Exists))
      .count();
    if existing_count > 0 {
      warnings.push(format!("{existing_count} files already exist"));
    }

    let preview = preview::Manifest {
      files,
      total_size,
      conflicts,
      warnings
    };

    debug!("{preview:#?}");
    info!(
      "Preview generated: {} files ({}), {} conflicts, {} warnings",
      preview.files.len(),
      preview.estimated_size(),
      preview.conflicts.len(),
      preview.warnings.len()
    );

    self
      .event_sink
      .on_event(DownloadEvent::PreviewCompleted {
        file_count: preview.files.len(),
        total_size: preview.total_size,
        conflicts: preview.conflicts.len()
      })
      .await;

    Ok(preview)
  }

  /// Starts the download process with progress reporting.
  ///
  /// This method validates URLs (if not already done), handles existing files
  /// according to the overwrite policy, and starts concurrent downloads with
  /// progress reporting.
  ///
  /// # Returns
  ///
  /// Returns a `Reporter` that provides real-time updates on download
  /// progress.
  ///
  /// # Errors
  ///
  /// Returns the first error encountered during the download process.
  pub async fn start(&mut self) -> Result<progress::Reporter> {
    info!("Starting download process");

    self
      .event_sink
      .on_event(DownloadEvent::DownloadStarted {
        url_count: self.urls.len()
      })
      .await;

    // Validate URLs if not already done
    let validated_urls = match &self.validated_urls {
      Some(urls) => urls.clone(),
      None => {
        let urls = self.validate_urls().await?;
        self.validated_urls = Some(urls.clone());
        urls
      }
    };

    // Handle existing files according to policy
    let urls_to_download = self.handle_existing_files(validated_urls).await?;

    if urls_to_download.is_empty() {
      warn!("No files to download after handling existing files");
      return Ok(progress::Reporter::completed());
    }

    // Ensure target directory exists
    create_dir_all(&self.target_dir)
      .await
      .map_err(|e| Error::FileSystem {
        message: format!("Failed to create target directory: {e}")
      })?;

    // Create temporary directory for atomic operations
    let temp_dir = self.target_dir.join(".tmp_downloads");
    create_dir_all(&temp_dir)
      .await
      .map_err(|e| Error::FileSystem {
        message: format!("Failed to create temp directory: {e}")
      })?;

    // Create progress reporter
    let progress_reporter = progress::Reporter::new(urls_to_download.len());
    let progress_tx = progress_reporter.sender();

    // Prepare download tasks
    let mut tasks = Vec::new();
    for (index, validated_url) in urls_to_download.into_iter().enumerate() {
      let temp_path =
        temp_dir.join(format!("{}_{}", index, validated_url.filename));

      let task = DownloadTask {
        url: validated_url.parsed,
        temp_path,
        final_path: validated_url.target_path,
        index,
        client: self.client.clone(),
        config: self.config.clone(),
        progress_tx: progress_tx.clone(),
        event_sink: self.event_sink.clone()
      };

      tasks.push(task);
    }

    // Execute downloads directly (no background spawn)
    let executor = TaskExecutor::new(self.config.concurrency_limit);

    warn!("Before results");
    let results = executor.execute(tasks).await;
    warn!("After results");

    // Clean up temporary directory
    if let Err(e) = remove_dir_all(&temp_dir).await {
      error!("Failed to clean up temp directory: {}", e);
    }

    // Process results and send final event
    let mut success_count = 0;
    let mut failed_urls = Vec::new();
    for (index, result) in results.into_iter().enumerate() {
      match result {
        Ok(_) => success_count += 1,
        Err(e) => {
          error!("Download {} failed: {}", index, e);
          failed_urls.push(format!("Task {index}: {e}"));
        }
      }
    }

    self
      .event_sink
      .on_event(DownloadEvent::DownloadCompleted {
        successful: success_count,
        failed: failed_urls.len(),
        errors: failed_urls
      })
      .await;

    // Return the progress_reporter so it stays alive
    Ok(progress_reporter)
  }

  /// Validates and prepares all URLs for downloading.
  async fn validate_urls(&self) -> Result<Vec<validation::Url>> {
    validation::Url::new(
      self.urls.clone(),
      &self.target_dir,
      &self.config.filename_strategy,
      self.validator.clone()
    )
    .await
  }

  /// Handles existing files according to the configured overwrite policy.
  async fn handle_existing_files(
    &self,
    validated_urls: Vec<validation::Url>
  ) -> Result<Vec<validation::Url>> {
    let mut to_download = Vec::new();
    let mut existing_files = Vec::new();

    for validated in validated_urls {
      if validated.exists {
        existing_files.push(validated.clone());
      }

      match self.config.overwrite_policy {
        OverwritePolicy::Skip =>
          if !validated.exists {
            to_download.push(validated);
          },
        OverwritePolicy::Overwrite => {
          to_download.push(validated);
        }
        OverwritePolicy::Error => {
          if validated.exists {
            return Err(Error::FileExists(validated.target_path));
          }
          to_download.push(validated);
        }
        OverwritePolicy::Rename => {
          //TODO: Implement logic for auto-renaming
          to_download.push(validated);
        }
      }
    }

    if !existing_files.is_empty() {
      match self.config.overwrite_policy {
        crate::config::OverwritePolicy::Skip => {
          info!("Skipping {} existing files", existing_files.len());
        }
        crate::config::OverwritePolicy::Overwrite => {
          warn!("Will overwrite {} existing files", existing_files.len());
        }
        _ => {}
      }
    }

    Ok(to_download)
  }
}
