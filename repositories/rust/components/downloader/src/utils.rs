use crate::*;

/// Convenience function to create a downloader with default settings.
///
/// This is equivalent to `Downloader::with_defaults()` but provides a
/// more ergonomic API for simple use cases.
///
/// # Examples
///
/// ```rust,no_run
/// use downloader::download;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let urls = vec!["https://httpbin.org/json"];
/// let mut downloader = download(urls, "./downloads")?;
/// let _progress_rx = downloader.start().await?;
/// # Ok(())
/// # }
/// ```
pub fn download<S, P>(urls: Vec<S>, target_dir: P) -> Result<Downloader>
where
  S: AsRef<str>,
  P: AsRef<std::path::Path>
{
  Downloader::new(urls, target_dir)
}

/// Convenience function to create a downloader with custom configuration.
///
/// # Examples
///
/// ```rust,no_run
/// use downloader::{download_with_config, Config};
/// use std::time::Duration;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let config = Config::builder()
///     .timeout(Duration::from_secs(60))
///     .build();
///
/// let urls = vec!["https://httpbin.org/delay/30"];
/// let mut downloader = download_with_config(urls, "./downloads", config)?;
/// let _progress_rx = downloader.start().await?;
/// # Ok(())
/// # }
/// ```
pub fn download_with_config<S, P>(
  urls: Vec<S>,
  target_dir: P,
  config: Config
) -> Result<Downloader>
where
  S: AsRef<str>,
  P: AsRef<std::path::Path>
{
  Downloader::new_with_config(urls, target_dir, config)
}

/// Formats a file size in bytes into a human-readable string with appropriate
/// units.
///
/// The function converts the given size in bytes to a string representation
/// with units ranging from bytes (B) to terabytes (TB). It uses binary prefixes
/// for conversion (i.e., 1 KB = 1024 B).
///
/// # Arguments
///
/// * `size` - The size in bytes to format.
///
/// # Returns
///
/// A `String` representing the human-readable file size.
///
/// # Examples
///
/// ```
/// use downloader::utils::format_filesize;
/// let formatted_size = format_filesize(1536);
/// assert_eq!(formatted_size, "1.5 KB");
/// ```
pub fn format_filesize(size: u64) -> String {
  let units = ["B", "KB", "MB", "GB", "TB"];
  let mut size = size as f64;
  let mut idx = 0;
  while size > 1024.0 && idx < units.len() - 1 {
    size /= 1024.0;
    idx += 1;
  }
  format!("{:.1} {}", size, units[idx])
}

pub async fn fetch_content_length(
  client: &reqwest::Client,
  url: &reqwest::Url
) -> Option<u64> {
  match client.head(url.clone()).send().await {
    Ok(response) => response
      .headers()
      .get(reqwest::header::CONTENT_LENGTH)
      .and_then(|v| v.to_str().ok())
      .and_then(|s| s.parse().ok()),
    Err(e) => {
      debug!("Failed to fetch HEAD for {}: {}", url, e);
      None
    }
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
  use std::time::Duration;

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
}
