use std::{
  env::current_dir,
  path::{Path, PathBuf}
};

/// Represents the home directory for datasets under a data directory.
/// It optionally holds two levels of subdirectories: `base` and `category`.
#[derive(Debug, Clone)]
pub struct Home {
  /// Parent path, e.g., "/home/user/Downloads"
  pub parent: PathBuf,

  /// Top-level folder, e.g., "data"
  pub base: Option<String>,

  /// Second-level folder/category, e.g., "imdb"
  pub category: Option<String>
}

impl Default for Home {
  fn default() -> Self {
    let parent = directories::UserDirs::new()
      .and_then(|dirs| dirs.download_dir().map(|p| p.to_path_buf()))
      .or_else(|| std::env::var("CARGO_MANIFEST_DIR").ok().map(PathBuf::from))
      .unwrap_or_else(|| PathBuf::from("."));
    let base = Some("data".to_string());
    let category = Some("imdb".to_string());

    Self {
      parent,
      base,
      category
    }
  }
}

impl Home {
  pub fn new<
    Parent: Into<Option<PathBuf>>,
    Base: Into<Option<String>>,
    Category: Into<Option<String>>
  >(
    parent: Parent,
    base: Base,
    category: Category
  ) -> Self {
    let parent = parent.into().unwrap_or_else(|| PathBuf::from("."));
    Self {
      parent,
      base: base.into(),
      category: category.into()
    }
  }

  pub fn to_pathbuf(&self) -> PathBuf {
    let mut path = self.parent.clone();
    if let Some(base) = &self.base {
      path.push(base);
    }
    if let Some(category) = &self.category {
      path.push(category);
    }
    path
  }

  /// Sets the `parent` folder.
  pub fn with_parent(&mut self, name: impl Into<PathBuf>) -> &mut Self {
    self.parent = name.into();
    self
  }

  /// Removes the `parent` folder.
  pub fn without_parent(&mut self) -> &mut Self {
    self.parent = current_dir().unwrap_or(PathBuf::from("."));
    self
  }

  /// Sets the `base` folder.
  pub fn with_base(&mut self, name: impl Into<String>) -> &mut Self {
    self.base = Some(name.into());
    self
  }

  /// Removes the `base` folder.
  pub fn without_base(&mut self) -> &mut Self {
    self.base = None;
    self
  }

  /// Sets the `category` folder.
  pub fn with_category(&mut self, name: impl Into<String>) -> &mut Self {
    self.category = Some(name.into());
    self
  }

  /// Removes the `category` folder in-place.
  pub fn without_category(&mut self) -> &mut Self {
    self.category = None;
    self
  }
}

#[derive(Debug, Clone)]
pub struct Paths {
  pub home: Home,
  pub source: PathBuf,
  pub import: PathBuf
}

impl Default for Paths {
  fn default() -> Self {
    let home = Home::default();
    Self {
      home: home.clone(),
      source: home.to_pathbuf().join("source"),
      import: home.to_pathbuf().join("import")
    }
  }
}
