//! Enhanced download task execution with retry logic and progress reporting
//!
//! This module provides robust download task execution with comprehensive
//! error handling, retry mechanisms, and detailed progress reporting.

use crate::*;
use std::{
  path::PathBuf,
  sync::Arc,
  time::{Duration, Instant},
};
use tokio::{
  fs::{File, rename},
  io::AsyncWriteExt,
  sync::Semaphore,
  time::sleep,
};

/// Represents a single download task with comprehensive metadata and
/// capabilities.
///
/// This enhanced version includes retry logic, progress reporting, and
/// detailed error handling compared to the original simple version.
#[derive(Debug, Clone)]
pub struct DownloadTask {
  /// The URL to download from
  pub url: reqwest::Url,
  /// Temporary file path for atomic operations
  pub temp_path: PathBuf,
  /// Final destination path
  pub final_path: PathBuf,
  /// Task index for identification and logging
  pub index: usize,
  /// HTTP client with configured settings
  pub client: reqwest::Client,
  /// Configuration settings
  pub config: crate::Config,
  /// Progress reporter
  pub progress_tx: progress::Sender,
  /// Event sink for notifications
  pub event_sink: Arc<dyn EventSink>,
}

impl DownloadTask {
  /// Executes the download task with retry logic and progress reporting.
  ///
  /// This method handles the complete download lifecycle:
  /// 1. Makes HTTP request with custom headers
  /// 2. Validates response and file size limits
  /// 3. Downloads with progress reporting
  /// 4. Handles errors and retries automatically
  /// 5. Atomically moves file to final location
  ///
  /// # Returns
  ///
  /// Returns `Ok(TaskResult)` on success or an error if all retry attempts
  /// fail.
  pub async fn execute(self) -> Result<TaskResult> {
    let start_time = Instant::now();
    let filename = self
      .final_path
      .file_name()
      .and_then(|n| n.to_str())
      .unwrap_or("unknown");

    // Notify that file download is starting
    self
      .event_sink
      .on_event(DownloadEvent::FileStarted {
        index: self.index,
        url: self.url.to_string(),
        filename: filename.to_string(),
      })
      .await;

    trace!(
      "Starting download task {}: {} -> {:?}",
      self.index, self.url, self.final_path
    );

    let mut last_error = None;
    let mut retry_count = 0;

    // Retry loop
    loop {
      match self.attempt_download().await {
        Ok(bytes_downloaded) => {
          let duration = start_time.elapsed();
          let final_speed = if duration.as_secs_f64() > 0.0 {
            bytes_downloaded as f64 / duration.as_secs_f64()
          } else {
            0.0
          };

          // Atomically move to final location
          if let Err(e) = rename(&self.temp_path, &self.final_path).await {
            let error = Error::FileSystem {
              message: format!("Failed to move file to final location: {e}"),
            };

            self
              .event_sink
              .on_event(DownloadEvent::FileFailed {
                index: self.index,
                url: self.url.to_string(),
                error: error.to_string(),
                retry_count,
              })
              .await;

            return Err(error);
          }

          let result = TaskResult {
            index: self.index,
            path: self.final_path.clone(),
            bytes_downloaded,
            duration,
            retry_count,
            final_speed,
          };

          // Report successful completion
          self
            .progress_tx
            .completed(self.index, bytes_downloaded as usize);

          self
            .event_sink
            .on_event(DownloadEvent::FileCompleted {
              index: self.index,
              filename: filename.to_string(),
              bytes_downloaded,
              duration,
            })
            .await;

          info!(
            "Download {} completed: {} ({} bytes in {:.2}s)",
            self.index,
            filename,
            bytes_downloaded,
            duration.as_secs_f64()
          );

          return Ok(result);
        }
        Err(e) => {
          error!(
            "Download {} attempt {} failed: {}",
            self.index,
            retry_count + 1,
            e
          );
          last_error = Some(e);
          retry_count += 1;

          if retry_count >= self.config.max_retries {
            break;
          }

          // Notify about retry
          self
            .event_sink
            .on_event(DownloadEvent::FileRetrying {
              index: self.index,
              url: self.url.to_string(),
              attempt: retry_count + 1,
              max_attempts: self.config.max_retries,
              delay: self.config.retry_delay,
            })
            .await;

          warn!(
            "Retrying download {} in {:.1}s (attempt {}/{})",
            self.index,
            self.config.retry_delay.as_secs_f64(),
            retry_count + 1,
            self.config.max_retries
          );

          sleep(self.config.retry_delay).await;
        }
      }
    }

    // All retries exhausted
    let final_error = last_error.unwrap_or_else(|| Error::TaskFailed {
      index: self.index,
      reason: "Unknown error after retries".to_string(),
    });

    self.progress_tx.failed(self.index, final_error.to_string());

    self
      .event_sink
      .on_event(DownloadEvent::FileFailed {
        index: self.index,
        url: self.url.to_string(),
        error: final_error.to_string(),
        retry_count,
      })
      .await;

    Err(final_error)
  }

  /// Attempts a single download without retry logic.
  async fn attempt_download(&self) -> Result<u64> {
    trace!("Attempting download for task {}: {}", self.index, self.url);

    // Build request with custom headers
    let mut request = self.client.get(self.url.clone());

    for (key, value) in &self.config.custom_headers {
      request = request.header(key, value);
    }

    // Make the HTTP request
    let response = request.send().await.map_err(|e| Error::RequestFailed {
      url: self.url.to_string(),
      download: e,
    })?;

    // Check response status
    if !response.status().is_success() {
      return Err(Error::HttpStatus {
        status: response.status().as_u16(),
        url: self.url.to_string(),
        message: "Failed to download file".to_string(),
      });
    }

    // Check content length and file size limits
    let content_length = response.content_length();
    if let (Some(max_size), Some(content_len)) =
      (self.config.max_file_size, content_length)
      && content_len > max_size
    {
      return Err(Error::FileTooLarge {
        size: content_len,
        max_size,
      });
    }

    debug!(
      "Task {}: Response OK, content-length: {:?}",
      self.index, content_length
    );

    // Download with progress reporting
    self.download_with_progress(response, content_length).await
  }

  /// Downloads response body with progress reporting.
  // In your DownloadTask::download_with_progress method, handle closed channels
  // gracefully:
  async fn download_with_progress(
    &self,
    response: reqwest::Response,
    content_length: Option<u64>,
  ) -> Result<u64> {
    use tokio_stream::StreamExt;

    // Create temporary file
    let mut temp_file =
      File::create(&self.temp_path)
        .await
        .map_err(|e| Error::FileSystem {
          message: format!("Failed to create temp file: {e}"),
        })?;

    let mut stream = response.bytes_stream();
    let mut bytes_downloaded = 0u64;
    let mut last_progress_report = Instant::now();

    while let Some(chunk_result) = stream.next().await {
      let chunk = chunk_result.map_err(|e| Error::RequestFailed {
        url: self.url.to_string(),
        download: e,
      })?;

      // Write chunk to file
      temp_file
        .write_all(&chunk)
        .await
        .map_err(|e| Error::FileSystem {
          message: format!("Failed to write to temp file: {e}"),
        })?;

      bytes_downloaded += chunk.len() as u64;

      // Report progress periodically
      let now = Instant::now();
      if now.duration_since(last_progress_report)
        >= self.config.progress_interval
      {
        let percentage = if let Some(total) = content_length {
          (bytes_downloaded as f64 / total as f64) * 100.0
        } else {
          0.0
        };

        // Send progress update
        self.progress_tx.progress(
          self.index,
          bytes_downloaded as usize,
          content_length.map(|c| c as usize),
        );

        // FIXED: Handle closed event sink gracefully
        self
          .event_sink
          .on_event(DownloadEvent::FileProgress {
            index: self.index,
            bytes_downloaded,
            total_bytes: content_length,
            percentage,
          })
          .await;

        last_progress_report = now;
      }

      // Check file size limit during download
      if let Some(max_size) = self.config.max_file_size
        && bytes_downloaded > max_size
      {
        return Err(Error::FileTooLarge {
          size: bytes_downloaded,
          max_size,
        });
      }
    }

    // Ensure all data is written to disk
    temp_file.flush().await.map_err(|e| Error::FileSystem {
      message: format!("Failed to flush temp file: {e}"),
    })?;

    drop(temp_file); // Close file handle

    trace!(
      "Task {}: Downloaded {} bytes to {:?}",
      self.index, bytes_downloaded, self.temp_path
    );

    Ok(bytes_downloaded)
  }
  // async fn download_with_progress(
  //   &self,
  //   response: reqwest::Response,
  //   content_length: Option<u64>
  // ) -> Result<u64> {
  //   use tokio_stream::StreamExt;

  //   // Create temporary file
  //   let mut temp_file =
  //     File::create(&self.temp_path)
  //       .await
  //       .map_err(|e| Error::FileSystem {
  //         message: format!("Failed to create temp file: {e}")
  //       })?;

  //   let mut stream = response.bytes_stream();
  //   let mut bytes_downloaded = 0u64;
  //   let mut last_progress_report = Instant::now();

  //   while let Some(chunk_result) = stream.next().await {
  //     let chunk = chunk_result.map_err(|e| Error::RequestFailed {
  //       url: self.url.to_string(),
  //       download: e
  //     })?;

  //     // Write chunk to file
  //     temp_file
  //       .write_all(&chunk)
  //       .await
  //       .map_err(|e| Error::FileSystem {
  //         message: format!("Failed to write to temp file: {e}")
  //       })?;

  //     bytes_downloaded += chunk.len() as u64;

  //     // Report progress periodically
  //     let now = Instant::now();
  //     if now.duration_since(last_progress_report)
  //       >= self.config.progress_interval
  //     {
  //       let percentage = if let Some(total) = content_length {
  //         (bytes_downloaded as f64 / total as f64) * 100.0
  //       } else {
  //         0.0
  //       };

  //       self.progress_tx.progress(
  //         self.index,
  //         bytes_downloaded as usize,
  //         content_length.map(|c| c as usize)
  //       );

  //       self
  //         .event_sink
  //         .on_event(DownloadEvent::FileProgress {
  //           index: self.index,
  //           bytes_downloaded,
  //           total_bytes: content_length,
  //           percentage
  //         })
  //         .await;

  //       last_progress_report = now;
  //     }

  //     // Check file size limit during download
  //     if let Some(max_size) = self.config.max_file_size
  //       && bytes_downloaded > max_size
  //     {
  //       return Err(Error::FileTooLarge {
  //         size: bytes_downloaded,
  //         max_size
  //       });
  //     }
  //   }

  //   // Ensure all data is written to disk
  //   temp_file.flush().await.map_err(|e| Error::FileSystem {
  //     message: format!("Failed to flush temp file: {e}")
  //   })?;

  //   drop(temp_file); // Close file handle

  //   trace!(
  //     "Task {}: Downloaded {} bytes to {:?}",
  //     self.index, bytes_downloaded, self.temp_path
  //   );

  //   Ok(bytes_downloaded)
  // }
}

/// Statistics about a completed download task.
#[derive(Debug, Clone)]
pub struct TaskResult {
  /// Task index
  pub index: usize,
  /// Final file path
  pub path: PathBuf,
  /// Number of bytes downloaded
  pub bytes_downloaded: u64,
  /// Time taken to complete the download
  pub duration: Duration,
  /// Number of retry attempts made
  pub retry_count: usize,
  /// Final download speed in bytes per second
  pub final_speed: f64,
}

/// Executes download tasks with configurable concurrency control.
///
/// This enhanced executor provides better redownload management and
/// monitoring compared to the original version.
#[derive(Debug)]
pub struct TaskExecutor {
  /// Optional concurrency limit
  concurrency_limit: Option<usize>,
}

impl TaskExecutor {
  /// Creates a new task executor with the specified concurrency limit.
  ///
  /// # Arguments
  ///
  /// * `concurrency_limit` - Maximum number of concurrent downloads (None =
  ///   unlimited)
  pub fn new(concurrency_limit: Option<usize>) -> Self {
    Self { concurrency_limit }
  }

  /// Executes a batch of download tasks with progress monitoring.
  ///
  /// This method manages the execution of multiple download tasks,
  /// respecting concurrency limits and providing comprehensive error reporting.
  ///
  /// # Arguments
  ///
  /// * `tasks` - Vector of download tasks to execute
  ///
  /// # Returns
  ///
  /// A vector of results, one for each task. The order matches the input task
  /// order.
  pub async fn execute(
    &self,
    tasks: Vec<DownloadTask>,
  ) -> Vec<Result<TaskResult>> {
    if tasks.is_empty() {
      return Vec::new();
    }

    info!(
      "Executing {} download tasks with concurrency limit: {:?}",
      tasks.len(),
      self.concurrency_limit
    );

    match self.concurrency_limit {
      Some(limit) => self.execute_with_semaphore(tasks, limit).await,
      None => self.execute_unlimited(tasks).await,
    }
  }

  // Fix for execute_with_semaphore method:
  async fn execute_with_semaphore(
    &self,
    tasks: Vec<DownloadTask>,
    limit: usize,
  ) -> Vec<Result<TaskResult>> {
    let semaphore = Arc::new(Semaphore::new(limit));
    let mut handles = Vec::with_capacity(tasks.len());

    debug!(
      "Using semaphore with {} permits for {} tasks",
      limit,
      tasks.len()
    );

    for task in tasks {
      let permit = semaphore.clone();
      let handle = tokio::spawn(async move {
        let _permit = permit.acquire().await.unwrap();
        let task_index = task.index; // ✅ Store index before moving task
        debug!("Task {} acquired semaphore permit", task_index);
        let result = task.execute().await;
        debug!("Task {} released semaphore permit", task_index); // ✅ Use stored index
        result
      });
      handles.push(handle);
    }

    let mut results = Vec::with_capacity(handles.len());

    // Remove problematic warn! statements
    for (index, handle) in handles.into_iter().enumerate() {
      match handle.await {
        Ok(task_result) => results.push(task_result),
        Err(join_error) => {
          error!("Task {} panicked: {}", index, join_error);
          results.push(Err(Error::TaskFailed {
            index,
            reason: join_error.to_string(),
          }));
        }
      }
    }

    results
  }

  // Fix for execute_unlimited method:
  async fn execute_unlimited(
    &self,
    tasks: Vec<DownloadTask>,
  ) -> Vec<Result<TaskResult>> {
    let mut handles = Vec::with_capacity(tasks.len());

    debug!("Executing {} tasks with unlimited concurrency", tasks.len());

    for task in tasks {
      let handle = tokio::spawn(async move {
        task.execute().await // ✅ Simple - no need to access task after this
      });
      handles.push(handle);
    }

    let mut results = Vec::with_capacity(handles.len());
    for (index, handle) in handles.into_iter().enumerate() {
      match handle.await {
        Ok(task_result) => results.push(task_result),
        Err(join_error) => {
          error!("Task {} panicked: {}", index, join_error);
          results.push(Err(Error::TaskFailed {
            index,
            reason: join_error.to_string(),
          }));
        }
      }
    }

    results
  }

  /// Executes tasks in batches to manage redownload usage.
  ///
  /// This method is useful for very large numbers of downloads where
  /// you want to process them in manageable chunks.
  pub async fn execute_in_batches(
    &self,
    tasks: Vec<DownloadTask>,
    batch_size: usize,
  ) -> Vec<Result<TaskResult>> {
    if tasks.is_empty() || batch_size == 0 {
      return Vec::new();
    }

    info!(
      "Executing {} tasks in batches of {}",
      tasks.len(),
      batch_size
    );

    let mut all_results = Vec::with_capacity(tasks.len());

    for (batch_num, batch) in tasks.chunks(batch_size).enumerate() {
      debug!(
        "Processing batch {} with {} tasks",
        batch_num + 1,
        batch.len()
      );

      let batch_tasks = batch.to_vec();
      let batch_results = self.execute(batch_tasks).await;
      all_results.extend(batch_results);

      // Optional delay between batches to be nice to servers
      if batch_num < (tasks.len() / batch_size) {
        sleep(Duration::from_millis(100)).await;
      }
    }

    all_results
  }
}

/// Builder for creating download tasks with validation.
#[derive(Debug)]
pub struct TaskBuilder {
  url: Option<reqwest::Url>,
  temp_path: Option<PathBuf>,
  final_path: Option<PathBuf>,
  index: usize,
  client: Option<reqwest::Client>,
  config: Option<crate::Config>,
  progress_tx: Option<progress::Sender>,
  event_sink: Option<Arc<dyn EventSink>>,
}

impl TaskBuilder {
  /// Creates a new task builder.
  pub fn new() -> Self {
    Self {
      url: None,
      temp_path: None,
      final_path: None,
      index: 0,
      client: None,
      config: None,
      progress_tx: None,
      event_sink: None,
    }
  }

  /// Sets the URL to download from.
  pub fn url(mut self, url: reqwest::Url) -> Self {
    self.url = Some(url);
    self
  }

  /// Sets the temporary file path.
  pub fn temp_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
    self.temp_path = Some(path.into());
    self
  }

  /// Sets the final destination path.
  pub fn final_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
    self.final_path = Some(path.into());
    self
  }

  /// Sets the task index.
  pub fn index(mut self, index: usize) -> Self {
    self.index = index;
    self
  }

  /// Sets the HTTP client.
  pub fn client(mut self, client: reqwest::Client) -> Self {
    self.client = Some(client);
    self
  }

  /// Sets the configuration.
  pub fn config(mut self, config: Config) -> Self {
    self.config = Some(config);
    self
  }

  /// Sets the progress sender.
  pub fn progress_sender(mut self, sender: progress::Sender) -> Self {
    self.progress_tx = Some(sender);
    self
  }

  /// Sets the event sink.
  pub fn event_sink(mut self, sink: Arc<dyn EventSink>) -> Self {
    self.event_sink = Some(sink);
    self
  }

  /// Builds the download task.
  ///
  /// # Errors
  ///
  /// Returns an error if any required fields are missing.
  pub fn build(self) -> Result<DownloadTask> {
    let url = self.url.ok_or_else(|| Error::Configuration {
      message: "URL is required".to_string(),
    })?;
    let temp_path = self.temp_path.ok_or_else(|| Error::Configuration {
      message: "Temp path is required".to_string(),
    })?;
    let final_path = self.final_path.ok_or_else(|| Error::Configuration {
      message: "Final path is required".to_string(),
    })?;
    let client = self.client.ok_or_else(|| Error::Configuration {
      message: "HTTP client is required".to_string(),
    })?;
    let config = self.config.ok_or_else(|| Error::Configuration {
      message: "Configuration is required".to_string(),
    })?;
    let progress_tx = self.progress_tx.ok_or_else(|| Error::Configuration {
      message: "Progress sender is required".to_string(),
    })?;
    let event_sink = self.event_sink.ok_or_else(|| Error::Configuration {
      message: "Event sink is required".to_string(),
    })?;

    Ok(DownloadTask {
      url,
      temp_path,
      final_path,
      index: self.index,
      client,
      config,
      progress_tx,
      event_sink,
    })
  }
}

impl Default for TaskBuilder {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{Config, events::NoOpEventSink, progress::Reporter};
  use tempfile::TempDir;

  #[tokio::test]
  async fn test_task_builder() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().join("temp_file");
    let final_path = temp_dir.path().join("final_file");
    let url = reqwest::Url::parse("https://httpbin.org/bytes/100").unwrap();

    let client = reqwest::Client::new();
    let config = Config::default();
    let progress_reporter = Reporter::new(1);
    let progress_tx = progress_reporter.sender();
    let event_sink = Arc::new(NoOpEventSink);

    let task = TaskBuilder::new()
      .url(url)
      .temp_path(temp_path)
      .final_path(final_path)
      .index(0)
      .client(client)
      .config(config)
      .progress_sender(progress_tx)
      .event_sink(event_sink)
      .build()
      .unwrap();

    assert_eq!(task.index, 0);
    assert_eq!(task.url.as_str(), "https://httpbin.org/bytes/100");
  }

  #[tokio::test]
  async fn test_task_executor_empty() {
    let executor = TaskExecutor::new(Some(2));
    let results = executor.execute(vec![]).await;
    assert!(results.is_empty());
  }

  #[test]
  fn test_task_builder_validation() {
    let result = TaskBuilder::new().build();
    assert!(result.is_err());

    if let Err(Error::Configuration { message }) = result {
      assert!(message.contains("URL is required"));
    } else {
      panic!("Expected Configuration");
    }
  }
}
