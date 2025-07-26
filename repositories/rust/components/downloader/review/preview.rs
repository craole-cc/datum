use crate::filename::FilenameExtractor;
use crate::{Error, Result};
use reqwest::Url;
use std::path::{Path, PathBuf};

/// Information about a file that will be downloaded
#[derive(Debug, Clone)]
pub struct DownloadPreview {
  /// The original URL
  pub url: String,
  /// The filename that will be used
  pub filename: String,
  /// The full path where the file will be saved
  pub destination: PathBuf,
  /// The index of this download in the batch
  pub index: usize,
}

impl DownloadPreview {
  /// Creates a new DownloadPreview instance
  pub fn new(
    url: String,
    filename: String,
    destination: PathBuf,
    index: usize,
  ) -> Self {
    Self {
      url,
      filename,
      destination,
      index,
    }
  }

  /// Display the preview information in a formatted way
  pub fn display(&self) -> String {
    format!(
      "  [{}] {} -> {}",
      self.index + 1,
      self.url,
      self.destination.display()
    )
  }

  /// Get detailed information about this download
  pub fn detailed_info(&self) -> String {
    format!(
      "File #{}\n  URL: {}\n  Filename: {}\n  Destination: {}\n  Directory exists: {}",
      self.index + 1,
      self.url,
      self.filename,
      self.destination.display(),
      self.destination.parent().map_or(false, |p| p.exists())
    )
  }
}

/// Generates preview information for downloads
#[derive(Debug, Clone)]
pub struct PreviewGenerator {
  home: PathBuf,
}

impl PreviewGenerator {
  /// Creates a new PreviewGenerator
  pub fn new<P: AsRef<Path>>(home: P) -> Self {
    Self {
      home: PathBuf::from(home.as_ref()),
    }
  }

  /// Generates a preview of what will be downloaded
  pub async fn generate_preview(
    &self,
    urls: &[String],
    filename_extractor: &FilenameExtractor,
  ) -> Result<Vec<DownloadPreview>> {
    let mut previews = Vec::new();

    for (index, url_str) in urls.iter().enumerate() {
      // Parse and validate URL
      let url = Url::parse(url_str).map_err(|_| Error::invalid_url(url_str))?;

      // Determine filename from URL
      let filename = filename_extractor.extract_filename(&url)?;
      let destination = self.home.join(&filename);

      previews.push(DownloadPreview::new(
        url_str.clone(),
        filename,
        destination,
        index,
      ));
    }

    Ok(previews)
  }

  /// Creates a formatted preview string
  pub fn format_preview(
    &self,
    previews: &[DownloadPreview],
    concurrency_limit: Option<usize>,
  ) -> Result<String> {
    let mut output = String::new();
    output.push_str("=== Download Preview ===\n");
    output
      .push_str(&format!("Destination directory: {}\n", self.home.display()));
    output.push_str(&format!("Number of files: {}\n", previews.len()));
    output.push_str(&format!("Concurrency limit: {:?}\n", concurrency_limit));
    output.push_str("\nFiles to download:\n");

    for preview in previews {
      output.push_str(&format!("{}\n", preview.display()));
    }

    Ok(output)
  }
}
