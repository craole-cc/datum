// -- Filesystem‐related Type Display (types/filesystem/display.rs) -- //
use super::*;

impl Display for Category {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::FileCreate => write!(f, "File creation"),
      Self::FileRead => write!(f, "File read"),
      Self::FileWrite => write!(f, "File write"),
      Self::FileDelete => write!(f, "File removal"),
      Self::FileCopy => write!(f, "File copy"),
      Self::FileMove => write!(f, "File move"),
      Self::DirCreate => write!(f, "Directory creation"),
      Self::DirRead => write!(f, "Directory read"),
      Self::DirDelete => write!(f, "Directory removal"),
      Self::NotFound => write!(f, "Path not found"),
      Self::PermissionDenied => write!(f, "Permission denied"),
      Self::AlreadyExists => write!(f, "Path already exists"),
      Self::Other => write!(f, "Other filesystem error"),
      Self::Multiple => write!(f, "Multiple errors encountered"),
      Self::Context => write!(f, "Filesystem context error"),
    }
  }
}
