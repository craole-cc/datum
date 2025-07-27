use std::{
  env::current_dir,
  path::{Path, PathBuf}
};

#[derive(Debug, Clone)]
pub struct Files {
  pub source: PathBuf,
  pub import: PathBuf
}

impl Default for Files {
  fn default() -> Self {
    let home = super::Home::default();
    Self {
      source: home.to_pathbuf().join("source"),
      import: home.to_pathbuf().join("import")
    }
  }
}
