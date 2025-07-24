use reqwest::{Url, get};
use tracing::trace;

use crate::{Error, error::Result};
use std::{
  fs::{File, create_dir_all, write},
  path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Downloader {
  pub source_url: String,
  pub target_path: PathBuf,
}

impl Downloader {
  pub fn new<S: AsRef<str>, P: AsRef<Path>>(
    source_url: S,
    target_path: P,
  ) -> Self {
    let source_url = String::from(source_url.as_ref());
    let target_path = PathBuf::from(target_path.as_ref());
    trace!(
      "Creating new Downloader with source: {}, target: {}",
      source_url,
      target_path.display()
    );
    Self {
      source_url,
      target_path,
    }
  }

  pub async fn fetch(&self) -> Result<()> {
    trace!(
      "Starting download from {} to {}",
      self.source_url,
      self.target_path.display()
    );

    // Validate the URL
    let url = self.source_url.parse::<Url>().map_err(|e| {
      trace!("Invalid URL: {}: {}", self.source_url, e);
      Error::InvalidURL(self.source_url.clone())
    });

    // Validate the target directory
    if let Some(parent) = self.target_path.parent() {
      if !parent.exists() {
        trace!("Creating directory: {parent:#?}");
        match create_dir_all(parent) {
          Ok(_) => {
            trace!("Successfully created directory: {parent:?}");
          }
          Err(e) => {
            trace!("Failed to create directory {parent:?}: {e:?}");
            return Err(Error::InvalidPath(parent.display().to_string()));
          }
        }
      }
    } else {
      trace!(
        "Invalid target path - no parent directory: {}",
        self.target_path.display()
      );
      return Err(Error::InvalidPath(self.target_path.display().to_string()));
    }

    // Send the request and read the response
    trace!("Sending HTTP request to {url:?}");
    let response = get(url?.as_str()).await?;
    let bytes = response.bytes().await?;
    trace!("Received {bytes:?} bytes");

    // Write the response to the file
    trace!("Writing response to file: {0:?}", self.target_path);
    write(self.target_path.as_path(), &bytes)?;

    trace!("Download completed successfully");
    Ok(())
  }
}

#[cfg(unix)]
fn platform_specific_invalid_path() -> std::path::PathBuf {
  // Unix test path
  "/root/forbidden/file.txt".into()
}

#[cfg(windows)]
fn platform_specific_invalid_path() -> std::path::PathBuf {
  // Windows test path; assumes Z: drive does not exist
  r"Z:\this_path_does_not_exist_and_is_not_creatable\file.txt".into()
}
