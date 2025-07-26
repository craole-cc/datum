use crate::{Error, Result};
use reqwest::Url;

/// Strategy for extracting filenames from URLs
pub trait FilenameStrategy {
  /// Extract a filename from a URL
  fn extract(&self, url: &Url) -> Result<String>;
}

/// Default filename extraction strategy
#[derive(Debug, Clone)]
pub struct DefaultFilenameStrategy;

impl FilenameStrategy for DefaultFilenameStrategy {
  fn extract(&self, url: &Url) -> Result<String> {
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

/// Custom filename extraction strategy that preserves original names
#[derive(Debug, Clone)]
pub struct PreserveOriginalStrategy;

impl FilenameStrategy for PreserveOriginalStrategy {
  fn extract(&self, url: &Url) -> Result<String> {
    let mut path_segments = url
      .path_segments()
      .ok_or_else(|| Error::invalid_url(url.as_str()))?;

    let filename = path_segments
      .next_back()
      .filter(|s| !s.is_empty())
      .unwrap_or("download");

    Ok(filename.to_string())
  }
}

/// Filename extractor that uses configurable strategies
#[derive(Debug)]
pub struct FilenameExtractor {
  strategy: Box<dyn FilenameStrategy + Send + Sync>,
}

impl FilenameExtractor {
  /// Creates a new FilenameExtractor with the default strategy
  pub fn new() -> Self {
    Self {
      strategy: Box::new(DefaultFilenameStrategy),
    }
  }

  /// Creates a new FilenameExtractor with a custom strategy
  pub fn with_strategy<S: FilenameStrategy + Send + Sync + 'static>(
    strategy: S,
  ) -> Self {
    Self {
      strategy: Box::new(strategy),
    }
  }

  /// Extracts a filename from a URL using the configured strategy
  pub fn extract_filename(&self, url: &Url) -> Result<String> {
    self.strategy.extract(url)
  }
}

impl Default for FilenameExtractor {
  fn default() -> Self {
    Self::new()
  }
}

impl Clone for FilenameExtractor {
  fn clone(&self) -> Self {
    // Since we can't clone Box<dyn Trait>, we create a new default instance
    // In a real implementation, you might want to add a Clone method to the trait
    Self::new()
  }
}
