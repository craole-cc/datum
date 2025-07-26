use crate::{
  Error, Result,
  task::{DownloadTask, TaskExecutor},
};
use reqwest::Url;
use std::{
  collections::HashMap,
  path::{Path, PathBuf},
  sync::Arc,
};
use tokio::fs::{create_dir_all as async_create_dir_all, remove_dir_all};

/// A concurrent file downloader that downloads multiple files from URLs to a local directory.
///
/// The downloader supports:
/// - Concurrent downloads with optional concurrency limiting
/// - Atomic file operations (downloads to temp files, then atomically renames)
/// - Automatic filename extraction from URLs
/// - Comprehensive error handling and logging
/// - Automatic directory creation
///
/// # Examples
///
/// ```rust,no_run
/// use downloader::Downloader;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let urls = vec![
///         "https://example.com/file1.txt",
///         "https://example.com/file2.pdf",
///     ];
///
///     // Download with concurrency limit of 3
///     let mut downloader = Downloader::new(urls.clone(), "/download/path", Some(3));
///     downloader.start(false).await?;
///
///     // Download without concurrency limit, forcing overwrites
///     let mut downloader = Downloader::new(urls, "/download/path", None);
///     downloader.start(true).await?;
///
/// #   Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Downloader {
  /// The list of URLs to download from
  pub urls: Vec<String>,

  /// The directory to place the downloaded files
  pub home: PathBuf,

  /// Optional limit on the number of concurrent downloads.
  /// If None, downloads will run with unlimited concurrency.
  pub concurrency_limit: Option<usize>,

  /// Indicates whether the target files already exist
  /// None = not checked yet, Some(bool) = checked result
  pub already_exists: Option<bool>,
}

impl Downloader {
  /// Creates a new Downloader instance.
  ///
  /// # Arguments
  ///
  /// * `urls` - A vector of URLs to download. Each URL should be a valid HTTP/HTTPS URL.
  /// * `path` - The target directory where files will be saved.
  /// * `concurrency_limit` - Optional limit on concurrent downloads. Use `None` for unlimited.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use downloader::Downloader;
  /// let urls = vec!["https://example.com/file.txt"];
  /// let downloader = Downloader::new(urls, "/downloads", Some(5));
  /// ```
  pub fn new<S: AsRef<str>, P: AsRef<Path>>(
    urls: Vec<S>,
    path: P,
    concurrency_limit: Option<usize>,
  ) -> Self {
    let urls: Vec<_> =
      urls.into_iter().map(|s| s.as_ref().to_string()).collect();
    let home = PathBuf::from(path.as_ref());
    let downloader = Self {
      urls,
      home,
      concurrency_limit,
      already_exists: None, // Indicates "not checked yet"
    };
    info!("Initiated {downloader:#?}");
    downloader
  }

  /// Checks which target files already exist before downloading.
  ///
  /// Returns a HashMap of URLs and their corresponding target paths that already exist.
  /// This allows the caller to decide whether to proceed with downloads or skip existing files.
  ///
  /// # Returns
  ///
  /// Returns `Ok(HashMap<String, PathBuf>)` containing URLs and paths of existing files.
  ///
  /// # Errors
  ///
  /// Returns `Error::InvalidURL` if any URL cannot be parsed.
  pub async fn check_existing_files(
    &mut self,
  ) -> Result<HashMap<String, PathBuf>> {
    let mut existing = HashMap::new();

    for url_str in &self.urls {
      let url = Url::parse(url_str).map_err(|_| Error::invalid_url(url_str))?;
      let filename = self.extract_filename(&url)?;
      let target_path = self.home.join(&filename);

      if target_path.exists() {
        #[derive(Debug)]
        struct ExistingDownloadTarget<'a> {
          link: &'a str,
          file: &'a Path,
        }

        warn!(
          "{existing_file:#?}",
          existing_file = ExistingDownloadTarget {
            link: url.as_str(),
            file: &target_path,
          }
        );
        existing.insert(url_str.clone(), target_path);
      }
    }

    // Set the cached result based on whether any files exist
    self.already_exists = Some(!existing.is_empty());
    Ok(existing)
  }

  /// Starts the download process for all configured URLs.
  ///
  /// This method:
  /// 1. Optionally checks for existing files (unless force is true)
  /// 2. Creates the target directory if it doesn't exist
  /// 3. Creates a temporary directory for atomic operations
  /// 4. Downloads all files concurrently (respecting concurrency limits)
  /// 5. Atomically moves completed downloads to their final locations
  /// 6. Cleans up temporary files
  ///
  /// # Arguments
  ///
  /// * `force` - If true, downloads will overwrite existing files without checking.
  ///   If false, returns an error if any target files already exist.
  ///
  /// # Returns
  ///
  /// Returns `Ok(())` if all downloads succeed, or the first error encountered.
  ///
  /// # Errors
  ///
  /// This method can return several types of errors:
  /// - `Error::InvalidURL` - If any URL cannot be parsed
  /// - `Error::RequestFailed` - If any HTTP request fails
  /// - `Error::WriteFailed` - If file system operations fail
  /// - `Error::HttpError` - If the server returns a non-success status code
  /// - `Error::TaskFailed` - If a download task panics or is cancelled
  /// - `Error::ExistingFiles` - If files exist and force is false
  pub async fn start(&mut self, force: bool) -> Result<()> {
    trace!("Starting download process with force={}", force);

    // Check for existing files if not forcing overwrites
    if !force {
      // Only check if we haven't already checked, or if we need to recheck
      let existing = if self.already_exists.is_none() {
        // Haven't checked yet, so check now
        self.check_existing_files().await?
      } else if self.already_exists == Some(true) {
        // We know files exist, but get the specific mapping
        // This is a bit inefficient, but ensures consistency
        self.check_existing_files().await?
      } else {
        // We know no files exist, so return empty map
        HashMap::new()
      };

      if !existing.is_empty() {
        return Err(Error::existing_files(existing));
      }
    } else {
      trace!("Force mode enabled - will overwrite existing files");
    }

    // Validate all URLs before starting downloads
    trace!("Validating {} URLs", self.urls.len());
    for (index, url_str) in self.urls.iter().enumerate() {
      Url::parse(url_str)
        .map_err(|_| Error::invalid_url(url_str))
        .map(|_| debug!("URL {} validated: {}", index, url_str))?;
    }

    // Create the target directory if it doesn't exist
    trace!("Creating target directory: {:?}", self.home);
    async_create_dir_all(&self.home).await.map_err(|e| {
      trace!("Failed to create target directory: {}", e);
      Error::WriteFailed(e)
    })?;

    // Create temporary directory for downloads
    let temp_dir = self.home.join(".tmp_downloads");
    trace!("Creating temporary directory: {:?}", temp_dir);
    async_create_dir_all(&temp_dir).await.map_err(|e| {
      trace!("Failed to create temp directory: {}", e);
      Error::WriteFailed(e)
    })?;

    // Prepare download tasks
    let mut tasks = Vec::with_capacity(self.urls.len());
    trace!("Preparing {} download tasks", self.urls.len());

    for (index, url_str) in self.urls.iter().enumerate() {
      // We already validated URLs above, so this should not fail
      let url = Url::parse(url_str).map_err(|_| Error::invalid_url(url_str))?;
      let filename = self.extract_filename(&url)?;

      let temp_path = temp_dir.join(format!("{index}_{filename}"));
      let final_path = self.home.join(&filename);

      debug!("Task {}: {} -> {:?}", index, url_str, final_path);

      let download_task = DownloadTask::new(url, temp_path, final_path, index);
      tasks.push(download_task);
    }

    // Execute downloads with concurrency control
    trace!(
      "Starting downloads with concurrency limit: {:?}",
      self.concurrency_limit
    );
    let executor = TaskExecutor::new(self.concurrency_limit);
    let results = executor.execute(tasks).await;

    // Clean up temporary directory
    trace!("Cleaning up temporary directory: {:?}", temp_dir);
    if let Err(e) = remove_dir_all(&temp_dir).await {
      error!("Warning: Failed to remove temp directory: {}", e);
      // Don't fail the entire operation for cleanup issues
    }

    // Check results - return first error encountered
    let mut success_count = 0;
    for (index, result) in results.into_iter().enumerate() {
      match result {
        Ok(_) => {
          success_count += 1;
          info!("Download {} completed successfully", index);
        }
        Err(e) => {
          error!("Download {} failed: {}", index, e);
          return Err(e);
        }
      }
    }

    info!("All {} downloads completed successfully", success_count);
    Ok(())
  }

  /// Extracts a filename from a URL's path.
  ///
  /// This method looks at the last path segment of the URL and uses it as the filename.
  /// If the URL has no path segments, or the last segment is empty, it defaults to "download".
  /// If the filename has no extension, ".bin" is appended.
  ///
  /// # Arguments
  ///
  /// * `url` - The URL to extract the filename from
  ///
  /// # Returns
  ///
  /// Returns the extracted filename, or an error if the URL has no path segments.
  ///
  /// # Examples
  ///
  /// - `https://example.com/path/file.txt` → `"file.txt"`
  /// - `https://example.com/data` → `"data.bin"`
  /// - `https://example.com/` → `"download.bin"`
  fn extract_filename(&self, url: &Url) -> Result<String> {
    let mut path_segments = url
      .path_segments()
      .ok_or_else(|| Error::invalid_url(url.as_str()))?;

    let filename = path_segments
      .next_back()
      .filter(|s| !s.is_empty())
      .unwrap_or("download");

    // If filename has no extension and looks like a generic name, add .bin
    let filename = if filename == "download" || !filename.contains('.') {
      format!("{filename}.bin")
    } else {
      filename.to_string()
    };

    Ok(filename)
  }
}
