use super::*;

/// Represents the home directory structure for datasets
///
/// The directory structure follows: parent/base/category/
/// Example: ~/Downloads/data/imdb/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Home {
  /// Parent path (e.g., "/home/user/Downloads")
  pub parent: PathBuf,
  /// Top-level folder (e.g., "data")
  pub base: Option<String>,
  /// Category folder (e.g., "imdb")
  pub category: Option<String>,
}

impl Home {
  /// Creates a new Home with explicit values
  pub fn new(
    parent: impl Into<PathBuf>,
    base: impl Into<Option<String>>,
    category: impl Into<Option<String>>,
  ) -> Self {
    Self {
      parent: parent.into(),
      base: base.into(),
      category: category.into(),
    }
  }

  /// Creates a Home with only the parent directory
  pub fn with_parent_only(parent: impl Into<PathBuf>) -> Self {
    Self {
      parent: parent.into(),
      base: None,
      category: None,
    }
  }

  /// Converts the Home to a complete PathBuf
  pub fn to_pathbuf(&self) -> PathBuf {
    let mut path = self.parent.clone();

    if let Some(ref base) = self.base {
      path.push(base);
    }

    if let Some(ref category) = self.category {
      path.push(category);
    }

    path
  }

  /// Builder method to set the parent directory
  pub fn with_parent(mut self, parent: impl Into<PathBuf>) -> Self {
    self.parent = parent.into();
    self
  }

  /// Builder method to set the base directory
  pub fn with_base(mut self, base: impl Into<String>) -> Self {
    self.base = Some(base.into());
    self
  }

  /// Builder method to remove the base directory
  pub fn without_base(mut self) -> Self {
    self.base = None;
    self
  }

  /// Builder method to set the category directory
  pub fn with_category(mut self, category: impl Into<String>) -> Self {
    self.category = Some(category.into());
    self
  }

  /// Builder method to remove the category directory
  pub fn without_category(mut self) -> Self {
    self.category = None;
    self
  }

  /// Creates all necessary directories
  pub fn create_dirs(&self) -> crate::Result<()> {
    let path = self.to_pathbuf();
    std::fs::create_dir_all(&path)
      .map_err(|_| crate::Error::PathCreation(path))?;
    Ok(())
  }

  /// Gets the default download directory, falling back to current directory or manifest dir
  fn default_parent() -> PathBuf {
    directories::UserDirs::new()
      .and_then(|dirs| dirs.download_dir().map(PathBuf::from))
      .or_else(|| std::env::var("CARGO_MANIFEST_DIR").ok().map(PathBuf::from))
      .unwrap_or_else(|| PathBuf::from("."))
  }
}

impl Default for Home {
  fn default() -> Self {
    Self {
      parent: Self::default_parent(),
      base: Some("data".to_string()),
      category: Some("imdb".to_string()),
    }
  }
}
