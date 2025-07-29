use super::*;

/// Represents a single IMDB dataset with metadata and file paths
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Dataset {
  pub name: String,
  pub description: String,
  pub url: String,
  pub files: Files,
}

impl Dataset {
  /// Creates a new dataset with the given parameters
  pub fn new(
    name: impl Into<String>,
    description: impl Into<String>,
    url: impl Into<String>,
    files: Files,
  ) -> Self {
    Self {
      name: name.into(),
      description: description.into(),
      url: url.into(),
      files,
    }
  }

  /// Gets the dataset name
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Gets the dataset description
  pub fn description(&self) -> &str {
    &self.description
  }

  /// Gets the dataset URL
  pub fn url(&self) -> &str {
    &self.url
  }

  /// Gets a reference to the files configuration
  pub fn files(&self) -> &Files {
    &self.files
  }

  /// Gets a mutable reference to the files configuration
  pub fn files_mut(&mut self) -> &mut Files {
    &mut self.files
  }

  /// Updates the dataset's file paths with a new home directory
  pub fn update_files_with_home(&mut self, home: &Home) -> crate::Result<()> {
    let download_filename = crate::Datasets::filename_from_url(&self.url)?;
    self.files =
      Files::with_home_and_filename(home, &self.name, download_filename)?;
    Ok(())
  }

  /// Updates the dataset description
  pub fn set_description(&mut self, description: impl Into<String>) {
    self.description = description.into();
  }

  /// Updates the dataset URL
  pub fn set_url(&mut self, url: impl Into<String>) {
    self.url = url.into();
  }

  /// Creates necessary directories for this dataset
  pub fn create_dirs(&self) -> crate::Result<()> {
    self.files.create_dirs()
  }

  /// Checks if the download file exists
  pub fn download_exists(&self) -> bool {
    self.files.download.exists()
  }

  /// Checks if the import file exists
  pub fn import_exists(&self) -> bool {
    self.files.import.exists()
  }
}
