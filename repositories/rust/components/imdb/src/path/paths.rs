use super::*;

use super::Home;
use std::path::PathBuf;

/// Collection of common paths derived from a Home directory
#[derive(Debug, Clone)]
pub struct Paths {
  pub home: Home,
  pub download: PathBuf,
  pub import: PathBuf,
}

impl Paths {
  /// Creates new Paths from a Home directory
  pub fn new(home: Home) -> Self {
    let home_path = home.to_pathbuf();
    Self {
      home,
      download: home_path.join("download"),
      import: home_path.join("import"),
    }
  }

  /// Creates all necessary directories
  pub fn create_dirs(&self) -> crate::Result<()> {
    self.home.create_dirs()?;
    std::fs::create_dir_all(&self.download)
      .map_err(|_| crate::Error::PathCreation(self.download.clone()))?;
    std::fs::create_dir_all(&self.import)
      .map_err(|_| crate::Error::PathCreation(self.import.clone()))?;
    Ok(())
  }
}

impl Default for Paths {
  fn default() -> Self {
    Self::new(Home::default())
  }
}

impl From<Home> for Paths {
  fn from(home: Home) -> Self {
    Self::new(home)
  }
}
