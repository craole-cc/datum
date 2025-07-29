use crate::*;

/// Functionality for handling dataset information (not actual downloading/extracting)
/// This focuses on metadata and structure - actual download/extract should use separate libraries
pub trait DatasetInfo {
  /// Gets the download URL for the dataset
  fn download_url(&self) -> &str;

  /// Checks if the dataset appears to be downloaded locally
  fn appears_downloaded(&self) -> bool;

  /// Gets metadata about the dataset files
  fn file_info(&self) -> Result<FileInfo>;
}

/// Information about dataset files
#[derive(Debug, Clone)]
pub struct FileInfo {
  pub download_exists: bool,
  pub import_exists: bool,
  pub download_size: Option<u64>,
  pub import_size: Option<u64>,
}

impl DatasetInfo for Dataset {
  fn download_url(&self) -> &str {
    &self.url
  }

  fn appears_downloaded(&self) -> bool {
    self.download_exists()
  }

  fn file_info(&self) -> Result<FileInfo> {
    let download_exists = self.download_exists();
    let import_exists = self.import_exists();

    let download_size = if download_exists {
      let metadata = metadata(&self.files.download)?;
      Some(metadata.len())
    } else {
      None
    };

    let import_size = if import_exists {
      let metadata = metadata(&self.files.import)?;
      Some(metadata.len())
    } else {
      None
    };

    Ok(FileInfo {
      download_exists,
      import_exists,
      download_size,
      import_size,
    })
  }
}

impl DatasetInfo for Datasets {
  fn download_url(&self) -> &str {
    // Not meaningful for collection - could return empty string or panic
    ""
  }

  fn appears_downloaded(&self) -> bool {
    self.iter().all(|dataset| dataset.appears_downloaded())
  }

  fn file_info(&self) -> Result<FileInfo> {
    let mut total_download_size = 0u64;
    let mut total_import_size = 0u64;
    let mut any_download = false;
    let mut any_import = false;
    let mut all_download = true;
    let mut all_import = true;

    for dataset in self.iter() {
      let info = dataset.file_info()?;

      if info.download_exists {
        any_download = true;
        if let Some(size) = info.download_size {
          total_download_size += size;
        }
      } else {
        all_download = false;
      }

      if info.import_exists {
        any_import = true;
        if let Some(size) = info.import_size {
          total_import_size += size;
        }
      } else {
        all_import = false;
      }
    }

    Ok(FileInfo {
      download_exists: all_download,
      import_exists: all_import,
      download_size: if any_download {
        Some(total_download_size)
      } else {
        None
      },
      import_size: if any_import {
        Some(total_import_size)
      } else {
        None
      },
    })
  }
}
