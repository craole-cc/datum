use reqwest::Url;
use std::sync::Arc;
use tokio::fs::rename;
use tokio::io::AsyncWriteExt;
use tracing::trace;

use crate::{Error, Result};
use std::path::PathBuf;

/// Represents a single download task with all necessary paths and metadata.
///
/// This struct contains all the information needed to download a single file:
/// - The download URL
/// - Temporary file path (for atomic operations)
/// - Final destination path
/// - Index for logging and identification
#[derive(Debug)]
pub struct DownloadTask {
  /// The URL to download from
  pub url: Url,
  /// Temporary file path for atomic operations
  pub temp_path: PathBuf,
  /// Final destination path
  pub final_path: PathBuf,
  /// Task index for logging and identification
  pub index: usize,
}

impl DownloadTask {
  /// Creates a new download task.
  pub fn new(
    url: Url,
    temp_path: PathBuf,
    final_path: PathBuf,
    index: usize,
  ) -> Self {
    Self {
      url,
      temp_path,
      final_path,
      index,
    }
  }

  /// Downloads a single file from a URL to a temporary location, then atomically moves it.
  ///
  /// This method performs the actual download work:
  /// 1. Makes HTTP GET request to the URL
  /// 2. Checks for HTTP success status
  /// 3. Downloads response body to bytes
  /// 4. Writes bytes to temporary file
  /// 5. Atomically renames temporary file to final destination
  ///
  /// The atomic rename ensures that partially downloaded files are never visible
  /// in the final location, preventing corruption issues.
  ///
  /// # Returns
  ///
  /// Returns `Ok(())` on success, or an error if any step fails.
  pub async fn execute(self) -> Result<()> {
    trace!(
      "Starting download {}: {} -> {:?}",
      self.index, self.url, self.final_path
    );

    // Download the file
    let response = reqwest::get(self.url.clone()).await?;

    if !response.status().is_success() {
      return Err(Error::http_error(
        response.status().as_u16(),
        self.url.as_str(),
      ));
    }

    let bytes = response.bytes().await?;

    trace!("Downloaded {} bytes for file {}", bytes.len(), self.index);

    // Write to temporary file
    let mut temp_file = tokio::fs::File::create(&self.temp_path).await?;

    temp_file.write_all(&bytes).await?;
    temp_file.flush().await?;

    drop(temp_file); // Ensure file is closed before rename

    trace!("Wrote temporary file: {:?}", self.temp_path);

    // Atomic rename to final destination
    rename(&self.temp_path, &self.final_path).await?;

    trace!(
      "Successfully moved to final location: {:?}",
      self.final_path
    );

    Ok(())
  }
}

/// Handles the execution of download tasks with optional concurrency control.
pub struct TaskExecutor {
  concurrency_limit: Option<usize>,
}

impl TaskExecutor {
  /// Creates a new task executor.
  ///
  /// # Arguments
  ///
  /// * `concurrency_limit` - Optional limit on concurrent tasks. Use `None` for unlimited.
  pub fn new(concurrency_limit: Option<usize>) -> Self {
    Self { concurrency_limit }
  }

  /// Executes a vector of download tasks with concurrency control.
  ///
  /// # Arguments
  ///
  /// * `tasks` - Vector of download tasks to execute
  ///
  /// # Returns
  ///
  /// A vector of results, one for each download task.
  pub async fn execute(&self, tasks: Vec<DownloadTask>) -> Vec<Result<()>> {
    if let Some(limit) = self.concurrency_limit {
      trace!("Using concurrency limit: {}", limit);
      self.execute_with_semaphore(tasks, limit).await
    } else {
      trace!("Using unlimited concurrency");
      self.execute_unlimited(tasks).await
    }
  }

  /// Downloads files with a semaphore-based concurrency limit.
  ///
  /// This method creates a semaphore with the specified limit and ensures that
  /// no more than `limit` downloads run simultaneously.
  ///
  /// # Arguments
  ///
  /// * `tasks` - Vector of download tasks to execute
  /// * `limit` - Maximum number of concurrent downloads
  ///
  /// # Returns
  ///
  /// A vector of results, one for each download task.
  async fn execute_with_semaphore(
    &self,
    tasks: Vec<DownloadTask>,
    limit: usize,
  ) -> Vec<Result<()>> {
    use tokio::sync::Semaphore;

    let semaphore = Arc::new(Semaphore::new(limit));
    let mut handles = Vec::new();

    for task in tasks {
      let permit = semaphore.clone();
      let handle = tokio::spawn(async move {
        let _permit = permit.acquire().await.unwrap();
        task.execute().await
      });
      handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
      match handle.await {
        Ok(result) => results.push(result),
        Err(e) => results.push(Err(Error::from(e))),
      }
    }

    results
  }

  /// Downloads files with unlimited concurrency.
  ///
  /// This method spawns all download tasks immediately without any concurrency control.
  /// Use with caution for large numbers of URLs to avoid overwhelming the system or server.
  ///
  /// # Arguments
  ///
  /// * `tasks` - Vector of download tasks to execute
  ///
  /// # Returns
  ///
  /// A vector of results, one for each download task.
  async fn execute_unlimited(
    &self,
    tasks: Vec<DownloadTask>,
  ) -> Vec<Result<()>> {
    let mut handles = Vec::new();

    for task in tasks {
      let handle = tokio::spawn(task.execute());
      handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
      match handle.await {
        Ok(result) => results.push(result),
        Err(e) => results.push(Err(Error::from(e))),
      }
    }

    results
  }
}
