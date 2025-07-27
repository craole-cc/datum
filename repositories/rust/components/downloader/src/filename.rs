//! Filename extraction and generation strategies
//!
//! This module provides various strategies for generating filenames
//! from URLs, handling edge cases, conflicts, and user preferences.

use crate::{Error, Result};
use std::{
  ffi::OsStr,
  path::{Path, PathBuf}
};

/// Strategy for extracting filenames from URLs.
///
/// Different strategies provide varying levels of intelligence and
/// safety when determining what to name downloaded files.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Strategy {
  /// Simple extraction from URL path (original behavior)
  Simple,

  /// Smart extraction with fallbacks and sanitization
  #[default]
  Smart,

  /// Use sequential numbering (download_1.bin, download_2.bin, etc.)
  Sequential,

  /// Use URL hash as filename (useful for avoiding conflicts)
  UrlHash,

  /// Custom strategy with user-provided function
  Custom(fn(&reqwest::Url, usize) -> Result<String>)
}

impl Strategy {
  /// Extracts a filename from a URL using this strategy.
  ///
  /// # Arguments
  ///
  /// * `url` - The URL to extract filename from
  /// * `index` - The index of this URL in the download list (for sequential
  ///   naming)
  ///
  /// # Returns
  ///
  /// Returns the extracted filename or an error if extraction fails.
  pub fn extract_filename(
    &self,
    url: &reqwest::Url,
    index: usize
  ) -> Result<String> {
    match self {
      Strategy::Simple => extract_simple(url),
      Strategy::Smart => extract_smart(url, index),
      Strategy::Sequential => extract_sequential(index),
      Strategy::UrlHash => extract_url_hash(url),
      Strategy::Custom(func) => func(url, index)
    }
  }

  /// Creates a custom filename strategy from a function.
  pub fn custom(func: fn(&reqwest::Url, usize) -> Result<String>) -> Self {
    Strategy::Custom(func)
  }
}

/// Simple filename extraction (original behavior).
///
/// Extracts the last path segment and adds .bin if no extension.
fn extract_simple(url: &reqwest::Url) -> Result<String> {
  let mut path_segments = url
    .path_segments()
    .ok_or_else(|| Error::MissingFilename(url.to_string()))?;

  let filename = path_segments
    .next_back()
    .filter(|s| !s.is_empty())
    .unwrap_or("download");

  let filename = if filename == "download" || !filename.contains('.') {
    format!("{filename}.bin")
  } else {
    filename.to_string()
  };

  Ok(filename)
}

/// Smart filename extraction with comprehensive fallbacks and sanitization.
///
/// This strategy provides the most intelligent filename extraction:
/// - Handles URL-encoded characters
/// - Sanitizes invalid filesystem characters
/// - Provides intelligent fallbacks
/// - Handles edge cases gracefully
fn extract_smart(url: &reqwest::Url, index: usize) -> Result<String> {
  trace!("Extracting smart filename from: {url}");

  // Try multiple extraction methods in order of preference
  let candidates = vec![
    try_from_content_disposition(url),
    try_from_path_segments(url),
    try_from_query_params(url),
    try_from_domain_and_path(url),
  ];

  let mut filename = None;
  for candidate in candidates {
    if let Some(name) = candidate
      && is_valid_filename(&name)
    {
      filename = Some(name);
      break;
    }
  }

  let mut filename = filename.unwrap_or_else(|| format!("download_{index}"));

  // URL decode the filename
  filename = urlencoding::decode(&filename)
    .map(|s| s.into_owned())
    .unwrap_or(filename);

  // Sanitize the filename
  filename = sanitize_filename(&filename);

  // Ensure reasonable length
  if filename.len() > 255 {
    let extension = Path::new(&filename)
      .extension()
      .and_then(OsStr::to_str)
      .unwrap_or("");

    let max_stem_length = 255 - extension.len() - 1; // -1 for the dot
    let stem = &filename[..max_stem_length.min(filename.len())];
    filename = if extension.is_empty() {
      stem.to_string()
    } else {
      format!("{stem}.{extension}")
    };
  }

  // Add extension if missing
  if !filename.contains('.') {
    filename = infer_extension(&filename, url)
      .unwrap_or_else(|_| format!("{filename}.bin"));
  }

  // Final validation
  if is_reserved_filename(&filename) {
    filename = format!(
      "file_{}.{}",
      index,
      Path::new(&filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("bin")
    );
  }

  debug!("Extracted filename: {filename}");
  Ok(filename)
}

/// Sequential filename generation.
///
/// Generates filenames like download_0.bin, download_1.bin, etc.
fn extract_sequential(index: usize) -> Result<String> {
  Ok(format!("download_{index}.bin"))
}

/// URL hash-based filename generation.
///
/// Creates a filename based on the hash of the URL to ensure uniqueness.
fn extract_url_hash(url: &reqwest::Url) -> Result<String> {
  use std::collections::hash_map::DefaultHasher;
  use std::hash::{Hash, Hasher};

  let mut hasher = DefaultHasher::new();
  url.as_str().hash(&mut hasher);
  let hash = hasher.finish();

  // Try to preserve the original extension if possible
  let extension = try_from_path_segments(url)
    .and_then(|name| {
      Path::new(&name)
        .extension()
        .and_then(OsStr::to_str)
        .map(|s| s.to_string())
    })
    .unwrap_or_else(|| "bin".to_string());

  Ok(format!("{hash:016x}.{extension}"))
}

/// Attempts to extract filename from Content-Disposition header (future
/// enhancement).
fn try_from_content_disposition(_url: &reqwest::Url) -> Option<String> {
  // This would require making an HTTP HEAD request
  // For now, return None to fall back to other methods
  None
}

/// Attempts to extract filename from URL path segments.
fn try_from_path_segments(url: &reqwest::Url) -> Option<String> {
  url
    .path_segments()?
    .next_back()
    .filter(|s| !s.is_empty() && *s != "/")
    .map(|s| s.to_string())
}

/// Attempts to extract filename from query parameters.
fn try_from_query_params(url: &reqwest::Url) -> Option<String> {
  // Look for common query parameters that might contain filenames
  let query_params = url.query_pairs();

  for (key, value) in query_params {
    match key.as_ref() {
      "filename" | "name" | "file" | "download" =>
        if !value.is_empty() {
          return Some(value.to_string());
        },
      _ => continue
    }
  }

  None
}

/// Attempts to create a filename from domain and path information.
fn try_from_domain_and_path(url: &reqwest::Url) -> Option<String> {
  let host = url.host_str()?;
  let path = url.path();

  // Create a meaningful filename from the domain and path
  let domain_part = host.split('.').next().unwrap_or("download");
  let path_part = path.trim_start_matches('/').replace('/', "_");

  if path_part.is_empty() {
    Some(domain_part.to_string())
  } else {
    Some(format!("{domain_part}_{path_part}"))
  }
}

/// Sanitizes a filename by removing or replacing invalid characters.
fn sanitize_filename(filename: &str) -> String {
  // Characters not allowed in filenames on various systems
  const INVALID_CHARS: &[char] =
    &['<', '>', ':', '"', '|', '?', '*', '/', '\\'];

  let mut sanitized = String::new();

  for ch in filename.chars() {
    if INVALID_CHARS.contains(&ch) || ch.is_control() {
      sanitized.push('_');
    } else {
      sanitized.push(ch);
    }
  }

  // Remove multiple consecutive underscores
  while sanitized.contains("__") {
    sanitized = sanitized.replace("__", "_");
  }

  // Trim underscores from start and end
  sanitized.trim_matches('_').to_string()
}

/// Checks if a filename is valid for the current filesystem.
fn is_valid_filename(filename: &str) -> bool {
  if filename.is_empty() || filename.len() > 255 {
    return false;
  }

  // Check for reserved names and invalid characters
  !is_reserved_filename(filename)
    && !filename
      .chars()
      .any(|c| c.is_control() || "<>:\"|?*\\/".contains(c))
}

/// Checks if a filename is reserved on common filesystems.
fn is_reserved_filename(filename: &str) -> bool {
  // Windows reserved names
  const RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6",
    "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6",
    "LPT7", "LPT8", "LPT9"
  ];

  let name_without_ext = Path::new(filename)
    .file_stem()
    .and_then(OsStr::to_str)
    .unwrap_or(filename)
    .to_uppercase();

  RESERVED_NAMES.contains(&name_without_ext.as_str())
}

/// Attempts to infer an appropriate file extension based on URL and content
/// hints.
fn infer_extension(filename: &str, url: &reqwest::Url) -> Result<String> {
  // Try to infer from URL path
  if let Some(path_ext) = url.path().rfind('.') {
    let ext = &url.path()[path_ext + 1..];
    if !ext.is_empty()
      && ext.len() <= 10
      && ext.chars().all(|c| c.is_alphanumeric())
    {
      return Ok(format!("{filename}.{ext}"));
    }
  }

  // Try to infer from domain or query parameters
  let url_str = url.as_str().to_lowercase();
  if url_str.contains("image")
    || url_str.contains("img")
    || url_str.contains("photo")
  {
    return Ok(format!("{filename}.jpg"));
  } else if url_str.contains("video") || url_str.contains("movie") {
    return Ok(format!("{filename}.mp4"));
  } else if url_str.contains("audio")
    || url_str.contains("music")
    || url_str.contains("sound")
  {
    return Ok(format!("{filename}.mp3"));
  } else if url_str.contains("doc") || url_str.contains("pdf") {
    return Ok(format!("{filename}.pdf"));
  } else if url_str.contains("archive") || url_str.contains("zip") {
    return Ok(format!("{filename}.zip"));
  }

  // Default fallback
  Ok(format!("{filename}.bin"))
}

/// A filename conflict resolver that can generate alternative names.
#[derive(Debug)]
pub struct ConflictResolver {
  strategy: ConflictStrategy
}

#[derive(Debug, Clone)]
pub enum ConflictStrategy {
  /// Add numeric suffix: file.txt -> file (1).txt
  NumericSuffix,

  /// Add timestamp suffix: file.txt -> file_20240127_143022.txt
  TimestampSuffix,

  /// Add hash suffix: file.txt -> file_a1b2c3d4.txt
  HashSuffix
}

impl ConflictResolver {
  /// Creates a new conflict resolver with the specified strategy.
  pub fn new(strategy: ConflictStrategy) -> Self {
    Self { strategy }
  }

  /// Resolves a filename conflict by generating an alternative name.
  pub fn resolve_conflict(
    &self,
    original_path: &Path,
    existing_files: &[PathBuf]
  ) -> PathBuf {
    match self.strategy {
      ConflictStrategy::NumericSuffix =>
        self.resolve_with_numeric_suffix(original_path, existing_files),
      ConflictStrategy::TimestampSuffix =>
        self.resolve_with_timestamp_suffix(original_path),
      ConflictStrategy::HashSuffix =>
        self.resolve_with_hash_suffix(original_path),
    }
  }

  fn resolve_with_numeric_suffix(
    &self,
    original_path: &Path,
    existing_files: &[PathBuf]
  ) -> PathBuf {
    let parent = original_path.parent().unwrap_or_else(|| Path::new(""));
    let stem = original_path
      .file_stem()
      .and_then(OsStr::to_str)
      .unwrap_or("file");
    let extension = original_path
      .extension()
      .and_then(OsStr::to_str)
      .unwrap_or("");

    let mut counter = 1;
    loop {
      let new_filename = if extension.is_empty() {
        format!("{stem} ({counter})")
      } else {
        format!("{stem} ({counter}).{extension}")
      };

      let new_path = parent.join(&new_filename);

      if !existing_files.contains(&new_path) && !new_path.exists() {
        return new_path;
      }

      counter += 1;
      if counter > 9999 {
        warn!("Could not resolve filename conflict after 9999 attempts");
        break;
      }
    }

    // Fallback: use timestamp
    self.resolve_with_timestamp_suffix(original_path)
  }

  fn resolve_with_timestamp_suffix(&self, original_path: &Path) -> PathBuf {
    let parent = original_path.parent().unwrap_or_else(|| Path::new(""));
    let stem = original_path
      .file_stem()
      .and_then(OsStr::to_str)
      .unwrap_or("file");
    let extension = original_path.extension().and_then(OsStr::to_str);

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();

    let new_filename = if let Some(ext) = extension {
      format!("{stem}_{timestamp}.{ext}")
    } else {
      format!("{stem}_{timestamp}")
    };

    parent.join(new_filename)
  }

  fn resolve_with_hash_suffix(&self, original_path: &Path) -> PathBuf {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let parent = original_path.parent().unwrap_or_else(|| Path::new(""));
    let stem = original_path
      .file_stem()
      .and_then(OsStr::to_str)
      .unwrap_or("file");
    let extension = original_path.extension().and_then(OsStr::to_str);

    let mut hasher = DefaultHasher::new();
    original_path.to_string_lossy().hash(&mut hasher);
    std::time::SystemTime::now().hash(&mut hasher);
    let hash = hasher.finish();

    let hash_suffix = format!("{:08x}", hash & 0xFFFFFFFF);

    let new_filename = if let Some(ext) = extension {
      format!("{stem}_{hash_suffix}.{ext}")
    } else {
      format!("{stem}_{hash_suffix}")
    };

    parent.join(new_filename)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_filename_extraction() {
    let url = reqwest::Url::parse("https://example.com/path/file.txt").unwrap();
    let result = extract_simple(&url).unwrap();
    assert_eq!(result, "file.txt");

    let url = reqwest::Url::parse("https://example.com/path/file").unwrap();
    let result = extract_simple(&url).unwrap();
    assert_eq!(result, "file.bin");

    let url = reqwest::Url::parse("https://example.com/").unwrap();
    let result = extract_simple(&url).unwrap();
    assert_eq!(result, "download.bin");
  }

  #[test]
  fn test_smart_filename_extraction() {
    let url = reqwest::Url::parse("https://example.com/my%20file.pdf").unwrap();
    let result = extract_smart(&url, 0).unwrap();
    assert_eq!(result, "my_file.pdf");

    let url = reqwest::Url::parse("https://example.com/CON").unwrap();
    let result = extract_smart(&url, 5).unwrap();
    assert!(result.starts_with("file_5."));
  }

  #[test]
  fn test_sanitize_filename() {
    assert_eq!(sanitize_filename("file<>name.txt"), "file__name.txt");
    assert_eq!(sanitize_filename("file___name.txt"), "file_name.txt");
    assert_eq!(sanitize_filename("___file___"), "file");
  }

  #[test]
  fn test_reserved_filename_detection() {
    assert!(is_reserved_filename("CON"));
    assert!(is_reserved_filename("con.txt"));
    assert!(is_reserved_filename("COM1"));
    assert!(!is_reserved_filename("CONFIG"));
    assert!(!is_reserved_filename("normal_file.txt"));
  }

  #[test]
  fn test_sequential_extraction() {
    assert_eq!(extract_sequential(0).unwrap(), "download_0.bin");
    assert_eq!(extract_sequential(42).unwrap(), "download_42.bin");
  }

  #[test]
  fn test_url_hash_extraction() {
    let url = reqwest::Url::parse("https://example.com/file.pdf").unwrap();
    let result = extract_url_hash(&url).unwrap();
    assert!(result.ends_with(".pdf"));
    assert!(result.len() > 20); // Hash + extension
  }
}
