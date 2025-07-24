mod helper;
pub use helper::*;

#[cfg(test)]
mod tests {
  use crate::*;
  use downloader::{Downloader, Error};
  use std::{fs, path::PathBuf};

  #[tokio::test]
  async fn test_download_valid_url() {
    let target_path = valid_path("dummy");
    let url = valid_url();

    eprintln!("Testing with URL: {url}");
    eprintln!("Target path: {target_path:?}");

    let downloader = Downloader::new(url, &target_path);
    let result = downloader.fetch().await;

    match &result {
      Ok(_) => eprintln!("Download succeeded"),
      Err(e) => eprintln!("Download failed with error: {e:?}"),
    }

    assert!(result.is_ok(), "Download failed: {:?}", result.err());

    // Check file exists and has expected size
    let metadata = fs::metadata(&target_path).expect("File not found");
    let actual_size = metadata.len();
    assert_eq!(
      actual_size, 20,
      "Expected file size 20 bytes, got {actual_size} bytes"
    );
  }

  #[tokio::test]
  async fn test_download_invalid_url() {
    let target_path = valid_path("dummy");
    let url = invalid_url();

    eprintln!("Testing with URL: {url}");
    eprintln!("Target path: {target_path:?}");

    let downloader = Downloader::new(url, &target_path);
    let result = downloader.fetch().await;

    assert!(matches!(result, Err(Error::InvalidURL(_))));
  }

  #[tokio::test]
  async fn test_download_to_invalid_path() {
    let target_path = invalid_path();
    let url = valid_url();

    let downloader = Downloader::new(url, &target_path);
    let result = downloader.fetch().await;

    assert!(matches!(result, Err(Error::InvalidPath(_))));
  }
}
