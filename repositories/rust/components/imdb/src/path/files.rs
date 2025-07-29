use super::*;

/// Represents download, raw, and import file paths for a dataset
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Files {
  pub download: PathBuf,
  pub raw: PathBuf,
  pub import: PathBuf,
}

impl Files {
  /// Creates new Files with explicit paths
  pub fn new(
    download: impl Into<PathBuf>,
    raw: impl Into<PathBuf>,
    import: impl Into<PathBuf>,
  ) -> Self {
    Self {
      download: download.into(),
      raw: raw.into(),
      import: import.into(),
    }
  }

  /// Creates Files based on a Home directory and dataset name
  pub fn with_home_and_filename(
    home: &Home,
    dataset_name: &str,
    download_filename: &str,
  ) -> Result<Self> {
    let home_path = home.to_pathbuf();

    let download = home_path.join("download").join(download_filename);

    // Create raw path in same directory as download, but with .tsv extension
    let raw = if download_filename.ends_with(".tsv.gz") {
      home_path
        .join("download")
        .join(download_filename.strip_suffix(".gz").unwrap())
    } else {
      // If not .tsv.gz, assume we want .tsv
      home_path.join("download").join(format!(
        "{}.tsv",
        download_filename
          .strip_suffix(&format!(
            ".{}",
            download_filename.split('.').next_back().unwrap_or("")
          ))
          .unwrap_or(download_filename)
      ))
    };

    let import = home_path
      .join("import")
      .join(dataset_name)
      .with_extension("parquet");

    Ok(Self {
      download,
      raw,
      import,
    })
  }

  /// Creates Files with default structure from Home
  pub fn from_home(home: &Home) -> Self {
    let home_path = home.to_pathbuf();
    Self {
      download: home_path.join("download"),
      raw: home_path.join("download"), // Raw defaults to same as download directory
      import: home_path.join("import"),
    }
  }

  /// Creates the necessary directories for download, raw, and import paths
  pub fn create_dirs(&self) -> Result<()> {
    if let Some(download_dir) = self.download.parent() {
      std::fs::create_dir_all(download_dir)
        .map_err(|_| crate::Error::PathCreation(download_dir.to_path_buf()))?;
    }

    if let Some(raw_dir) = self.raw.parent() {
      std::fs::create_dir_all(raw_dir)
        .map_err(|_| crate::Error::PathCreation(raw_dir.to_path_buf()))?;
    }

    if let Some(import_dir) = self.import.parent() {
      std::fs::create_dir_all(import_dir)
        .map_err(|_| crate::Error::PathCreation(import_dir.to_path_buf()))?;
    }

    Ok(())
  }
}

impl Default for Files {
  fn default() -> Self {
    Self::from_home(&Home::default())
  }
}

impl From<&Home> for Files {
  fn from(home: &Home) -> Self {
    Self::from_home(home)
  }
}
