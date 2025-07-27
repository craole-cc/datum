use crate::*;
use std::path::PathBuf;

/// Preview information about planned downloads
#[derive(Debug)]
pub struct Manifest {
  pub files: Vec<Target>,
  pub total_size: Option<u64>,
  pub conflicts: Vec<Conflict>,
  pub warnings: Vec<String>
}

impl Manifest {
  pub fn estimated_size(&self) -> String {
    if let Some(total_size) = self.total_size {
      format_filesize(total_size)
    } else {
      "Size unknown".to_string()
    }
  }
}

#[derive(Debug)]
pub struct Target {
  pub url: String,
  pub filename: String,
  pub target_path: PathBuf,
  pub estimated_size: Option<u64>,
  pub status: Status
}

#[derive(Debug)]
pub enum Status {
  Ready,
  Exists,
  InvalidUrl,
  TooLarge,
  AccessDenied
}

impl Status {
  pub async fn new(
    max_file_size: Option<u64>,
    validated: &validation::Url,
    size: Option<u64>
  ) -> Self {
    if validated.exists {
      return Self::Exists;
    }

    if let Some(max_size) = max_file_size
      && let Some(file_size) = size
      && file_size > max_size
    {
      return Self::TooLarge;
    }

    //TODO: Could add more status checks here (permissions, disk space, etc.)
    Self::Ready
  }
}

#[derive(Debug)]
pub struct Conflict {
  pub filename: String,
  pub urls: Vec<String>
}
