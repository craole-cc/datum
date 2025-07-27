use std::path::PathBuf;

#[derive(Debug)]
pub struct Extractor {
  pub source_path: PathBuf,
  pub target_path: PathBuf
}

// pub enum Error {
//   FileSystem(std::io::Error),
//   ArchiveError(zip::result::ZipError),
// }
