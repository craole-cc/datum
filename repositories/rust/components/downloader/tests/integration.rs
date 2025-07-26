mod helper;
pub use helper::*;

#[cfg(test)]
mod tests {
  use crate::*;
  use downloader::{Downloader, Error};
  use std::{fs, path::PathBuf};
  use tokio::test;

  #[test]
  async fn test_single_download_success() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let urls = vec![valid_url()];

    let downloader = Downloader::new(urls, temp_dir.path(), None);
    let result = downloader.start().await;

    assert!(result.is_ok(), "Download should succeed: {result:?}");

    // Check that file was created
    let files: Vec<_> = fs::read_dir(temp_dir.path())
      .expect("Failed to read directory")
      .collect();

    assert_eq!(files.len(), 1, "Should have exactly one file");

    let file_path = files[0].as_ref().unwrap().path();
    let content = fs::read(&file_path).expect("Failed to read downloaded file");
    assert_eq!(
      content.len(),
      20,
      "File should be 20 bytes (as per httpbin.org/bytes/20)"
    );
  }

  #[test]
  async fn test_multiple_downloads_success() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let urls = vec![
      "https://httpbin.org/bytes/10",
      "https://httpbin.org/bytes/20",
      "https://httpbin.org/bytes/30",
    ];

    let downloader = Downloader::new(urls, temp_dir.path(), Some(2));
    let result = downloader.start().await;

    assert!(result.is_ok(), "Download should succeed: {result:?}");

    // Check that all files were created
    let files: Vec<_> = fs::read_dir(temp_dir.path())
      .expect("Failed to read directory")
      .collect();

    assert_eq!(files.len(), 3, "Should have exactly three files");

    // Verify file sizes
    let mut sizes = Vec::new();
    for file_entry in files {
      let file_path = file_entry.unwrap().path();
      let content =
        fs::read(&file_path).expect("Failed to read downloaded file");
      sizes.push(content.len());
    }

    sizes.sort();
    assert_eq!(sizes, vec![10, 20, 30], "Files should have correct sizes");
  }

  #[test]
  async fn test_invalid_url_error() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let urls = vec![invalid_url()];

    let downloader = Downloader::new(urls, temp_dir.path(), None);
    let result = downloader.start().await;

    assert!(result.is_err(), "Should fail with invalid URL");

    match result.unwrap_err() {
      Error::InvalidURL(_) => {} // Expected
      other => panic!("Expected InvalidURL error, got: {other:?}"),
    }
  }

  #[test]
  async fn test_http_error() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let urls = vec!["https://httpbin.org/status/404"];

    let downloader = Downloader::new(urls, temp_dir.path(), None);
    let result = downloader.start().await;

    assert!(result.is_err(), "Should fail with HTTP 404");

    match result.unwrap_err() {
      Error::HttpError { status, url } => {
        assert_eq!(status, 404);
        assert!(url.contains("httpbin.org/status/404"));
      }
      other => panic!("Expected HttpError, got: {other:?}"),
    }
  }

  #[test]
  async fn test_filename_extraction_edge_cases() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");

    // Test various filename extraction scenarios
    let test_cases = vec![
      ("https://httpbin.org/bytes/5", "5.bin"), // Last path segment is "5"
      ("https://httpbin.org/json", "json.bin"), // No extension
    ];

    for (url, expected_suffix) in test_cases {
      let downloader = Downloader::new(vec![url], temp_dir.path(), None);
      let result = downloader.start().await;

      assert!(
        result.is_ok(),
        "Download should succeed for {url}: {result:?}"
      );

      // Find the created file
      let files: Vec<_> = fs::read_dir(temp_dir.path())
        .expect("Failed to read directory")
        .filter_map(|entry| entry.ok())
        .collect();

      // Should have exactly one file and it should end with expected suffix
      let found_file = files.iter().any(|file| {
        file
          .file_name()
          .to_string_lossy()
          .ends_with(expected_suffix)
      });

      assert!(
        found_file,
        "Should find file ending with '{}' for URL: {}. Found files: {:?}",
        expected_suffix,
        url,
        files.iter().map(|f| f.file_name()).collect::<Vec<_>>()
      );

      // Clean up for next iteration
      for file in files {
        fs::remove_file(file.path()).expect("Failed to clean up test file");
      }
    }
  }

  #[test]
  async fn test_concurrent_downloads_different_sizes() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let urls = vec![
      "https://httpbin.org/bytes/100",
      "https://httpbin.org/bytes/200",
      "https://httpbin.org/bytes/50",
      "https://httpbin.org/bytes/150",
    ];

    let downloader = Downloader::new(urls, temp_dir.path(), Some(2));
    let result = downloader.start().await;

    assert!(result.is_ok(), "Download should succeed: {result:?}");

    // Check that all files were created
    let files: Vec<_> = fs::read_dir(temp_dir.path())
      .expect("Failed to read directory")
      .collect();

    assert_eq!(files.len(), 4, "Should have exactly four files");

    // Verify total downloaded bytes
    let mut total_bytes = 0;
    for file_entry in files {
      let file_path = file_entry.unwrap().path();
      let content =
        fs::read(&file_path).expect("Failed to read downloaded file");
      total_bytes += content.len();
    }

    assert_eq!(total_bytes, 500, "Total bytes should be 100+200+50+150=500");
  }

  #[test]
  async fn test_duplicate_filenames_handling() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    // These URLs will both extract to the same filename
    let urls = vec![
      "https://httpbin.org/bytes/10",
      "https://httpbin.org/bytes/20",
    ];

    let downloader = Downloader::new(urls, temp_dir.path(), None);
    let result = downloader.start().await;

    // This should succeed
    assert!(result.is_ok(), "Download should succeed: {result:?}");

    let files: Vec<_> = fs::read_dir(temp_dir.path())
      .expect("Failed to read directory")
      .collect();

    // Print debug info to see what files were actually created
    println!(
      "Files created: {:?}",
      files
        .iter()
        .map(|f| f.as_ref().unwrap().file_name())
        .collect::<Vec<_>>()
    );

    // Both URLs have different last path segments ("10" vs "20"),
    // so they should create different files: "10.bin" and "20.bin"
    assert_eq!(
      files.len(),
      2,
      "Should have two files with different names: 10.bin and 20.bin"
    );

    // Verify both files exist and have correct sizes
    let mut sizes = Vec::new();
    for file_entry in files {
      let file_path = file_entry.unwrap().path();
      let content = fs::read(&file_path).expect("Failed to read file");
      sizes.push(content.len());
    }

    sizes.sort();
    assert_eq!(sizes, vec![10, 20], "Should have files of 10 and 20 bytes");
  }

  #[test]
  async fn test_true_duplicate_filenames() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    // These URLs will extract to the same filename because they have no path segments
    let urls = vec![
      "https://httpbin.org/", // Will become "download.bin"
      "https://httpbin.org/", // Will also become "download.bin"
    ];

    let downloader = Downloader::new(urls, temp_dir.path(), None);
    let result = downloader.start().await;

    // This should succeed - the second download overwrites the first
    assert!(result.is_ok(), "Download should succeed: {result:?}");

    let files: Vec<_> = fs::read_dir(temp_dir.path())
      .expect("Failed to read directory")
      .collect();

    // Should have only one file (the second overwrote the first)
    assert_eq!(
      files.len(),
      1,
      "Should have one file (second overwrote first)"
    );

    let file_path = files[0].as_ref().unwrap().path();
    let filename = file_path.file_name().unwrap().to_string_lossy();
    assert_eq!(
      filename, "download.bin",
      "File should be named download.bin"
    );
  }

  #[test]
  async fn test_large_file_download() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    // Download a 1KB file
    let urls = vec!["https://httpbin.org/bytes/1024"];

    let downloader = Downloader::new(urls, temp_dir.path(), None);
    let result = downloader.start().await;

    assert!(
      result.is_ok(),
      "Large file download should succeed: {result:?}"
    );

    let files: Vec<_> = fs::read_dir(temp_dir.path())
      .expect("Failed to read directory")
      .collect();

    assert_eq!(files.len(), 1, "Should have exactly one file");

    let file_path = files[0].as_ref().unwrap().path();
    let content = fs::read(&file_path).expect("Failed to read downloaded file");
    assert_eq!(content.len(), 1024, "File should be exactly 1024 bytes");
  }

  #[test]
  async fn test_temporary_directory_cleanup() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let urls = vec!["https://httpbin.org/bytes/10"];

    let downloader = Downloader::new(urls, temp_dir.path(), None);
    let result = downloader.start().await;

    assert!(result.is_ok(), "Download should succeed: {result:?}");

    // Check that temporary directory was cleaned up
    let tmp_dir = temp_dir.path().join(".tmp_downloads");
    assert!(
      !tmp_dir.exists(),
      "Temporary directory should be cleaned up"
    );

    // But the downloaded file should exist
    let files: Vec<_> = fs::read_dir(temp_dir.path())
      .expect("Failed to read directory")
      .collect();

    assert_eq!(files.len(), 1, "Downloaded file should exist");
  }

  #[test]
  async fn test_invalid_url_detailed() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let invalid_url = "not_a_url_at_all";
    let urls = vec![invalid_url];

    let downloader = Downloader::new(urls, temp_dir.path(), None);
    let result = downloader.start().await;

    assert!(result.is_err(), "Should fail with invalid URL");

    match result.unwrap_err() {
      Error::InvalidURL(url) => {
        assert_eq!(url, invalid_url, "Error should contain the invalid URL");
      }
      other => panic!("Expected InvalidURL error, got: {other:?}"),
    }
  }

  #[test]
  async fn test_empty_urls_list() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let urls: Vec<&str> = vec![];

    let downloader = Downloader::new(urls, temp_dir.path(), None);
    let result = downloader.start().await;

    assert!(result.is_ok(), "Empty download list should succeed");

    // Check that no files were created
    let files: Vec<_> = fs::read_dir(temp_dir.path())
      .expect("Failed to read directory")
      .collect();

    assert_eq!(files.len(), 0, "Should have no files");
  }
}
