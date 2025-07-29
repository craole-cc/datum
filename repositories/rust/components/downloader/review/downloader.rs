use reqwest::Url;
use std::sync::Arc;
use tokio::fs::{create_dir_all as async_create_dir_all, remove_dir_all};
use tracing::trace;

use crate::filename::FilenameExtractor;
use crate::preview::{DownloadPreview, PreviewGenerator};
use crate::task::{DownloadTask, TaskExecutor};
use crate::ui::{PreviewAction, UserInterface};
use crate::{Error, Result};
use std::path::{Path, PathBuf};

/// A concurrent file downloader that downloads multiple files from URLs to a local directory.
///
/// The downloader supports:
/// - Interactive preview mode to see what will be downloaded
/// - Concurrent downloads with optional concurrency limiting
/// - Atomic file operations (downloads to temp files, then atomically renames)
/// - Automatic filename extraction from URLs
/// - Comprehensive error handling and logging
/// - Automatic directory creation
///
/// # Examples
///
/// ```rust,no_run
/// use downloader::{Downloader, ConsoleInterface};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let urls = vec![
///         "https://example.com/file1.txt",
///         "https://example.com/file2.pdf",
///     ];
///
///     // Download with interactive preview
///     let mut downloader = Downloader::new(urls.clone(), "/download/path", Some(3));
///     let ui = ConsoleInterface::new();
///     downloader.start_with_preview(&ui).await?;
///
///     // Download without preview (original behavior)
///     downloader.start().await?;
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

  /// Preview generator for creating download previews
  preview_generator: PreviewGenerator,

  /// Filename extractor for determining filenames from URLs
  filename_extractor: FilenameExtractor,
}

impl Downloader {
  /// Creates a new Downloader instance.
  ///
  /// # Arguments
  ///
  /// * `urls` - A vector of URLs to download. Each URL should be a valid HTTP/HTTPS URL.
  /// * `path` - The target directory where files will be saved.
  /// * `concurrency_limit` - Optional limit on concurrent downloads. Use `None` for unlimited.
  pub fn new<S: AsRef<str>, P: AsRef<Path>>(
    urls: Vec<S>,
    path: P,
    concurrency_limit: Option<usize>,
  ) -> Self {
    let urls = urls.into_iter().map(|s| s.as_ref().to_string()).collect();
    let home = PathBuf::from(path.as_ref());

    trace!(
      "Creating new Downloader with download: {urls:?}, target: {home:#?} and concurrency limit: {concurrency_limit:?}",
    );

    Self {
      urls,
      home: home.clone(),
      concurrency_limit,
      preview_generator: PreviewGenerator::new(home),
      filename_extractor: FilenameExtractor::new(),
    }
  }

  /// Creates a new Downloader with custom components for dependency injection.
  pub fn with_components<S: AsRef<str>, P: AsRef<Path>>(
    urls: Vec<S>,
    path: P,
    concurrency_limit: Option<usize>,
    filename_extractor: FilenameExtractor,
  ) -> Self {
    let urls = urls.into_iter().map(|s| s.as_ref().to_string()).collect();
    let home = PathBuf::from(path.as_ref());

    Self {
      urls,
      home: home.clone(),
      concurrency_limit,
      preview_generator: PreviewGenerator::new(home),
      filename_extractor,
    }
  }

  /// Generates a preview of what will be downloaded without starting the downloads.
  pub async fn generate_preview(&self) -> Result<Vec<DownloadPreview>> {
    self
      .preview_generator
      .generate_preview(&self.urls, &self.filename_extractor)
      .await
  }

  /// Shows an interactive preview and starts downloads based on user choice.
  pub async fn start_with_preview<U: UserInterface>(
    &self,
    ui: &U,
  ) -> Result<()> {
    let previews = self.generate_preview().await?;

    // Show preview and get user choice
    let action = ui
      .show_preview_and_get_choice(
        &previews,
        &self.home,
        self.concurrency_limit,
      )
      .await?;

    match action {
      PreviewAction::Cancel => {
        ui.show_message("Download cancelled by user.").await;
        Ok(())
      }
      PreviewAction::ProceedAll => {
        ui.show_message(&format!(
          "Starting download of all {} files...",
          previews.len()
        ))
        .await;
        self.start().await
      }
      PreviewAction::ProceedSelected(indices) => {
        ui.show_message(&format!(
          "Starting download of {} selected files...",
          indices.len()
        ))
        .await;
        let selected_urls: Vec<String> =
          indices.iter().map(|&i| self.urls[i].clone()).collect();

        // Create a new downloader with only selected URLs
        let selected_downloader =
          Downloader::new(selected_urls, &self.home, self.concurrency_limit);
        selected_downloader.start().await
      }
      PreviewAction::ShowDetails => {
        ui.show_detailed_preview(&previews).await;
        // Recursively call for another choice
        self.start_with_preview(ui).await
      }
    }
  }

  /// Creates a non-interactive preview as a formatted string.
  pub async fn format_preview(&self) -> Result<String> {
    let previews = self.generate_preview().await?;
    self
      .preview_generator
      .format_preview(&previews, self.concurrency_limit)
  }

  /// Starts the download process for all configured URLs.
  pub async fn start(&self) -> Result<()> {
    trace!("Starting download process");

    // Create the target directory if it doesn't exist
    async_create_dir_all(&self.home).await?;

    // Create temporary directory for downloads
    let temp_dir = self.home.join(".tmp_downloads");
    async_create_dir_all(&temp_dir).await?;

    trace!("Created temporary directory: {:?}", temp_dir);

    // Prepare download tasks
    let mut tasks = Vec::new();

    for (index, url_str) in self.urls.iter().enumerate() {
      trace!("Processing URL {}: {}", index, url_str);

      // Parse and validate URL
      let url = Url::parse(url_str).map_err(|_| Error::invalid_url(url_str))?;

      // Determine filename from URL
      let filename = self.filename_extractor.extract_filename(&url)?;
      trace!("Extracted filename: {}", filename);

      let temp_path = temp_dir.join(format!("{index}_{filename}"));
      let final_path = self.home.join(&filename);

      let download_task = DownloadTask::new(url, temp_path, final_path, index);
      tasks.push(download_task);
    }

    // Execute downloads with concurrency control
    let executor = TaskExecutor::new(self.concurrency_limit);
    let results = executor.execute(tasks).await;

    // Clean up temporary directory
    trace!("Cleaning up temporary directory");
    if let Err(e) = remove_dir_all(&temp_dir).await {
      trace!("Warning: Failed to remove temp directory: {}", e);
    }

    // Check if all downloads succeeded
    for result in results {
      result?;
    }

    trace!("All downloads completed successfully");
    Ok(())
  }
}
