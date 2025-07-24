use std::path::PathBuf;
use tempfile::tempdir;

#[cfg(unix)]
pub fn invalid_path() -> PathBuf {
  "/root/forbidden/file.txt".into()
}

#[cfg(windows)]
pub fn invalid_path() -> PathBuf {
  r"Z:\this_path_does_not_exist_and_is_not_creatable\file.txt".into()
}

pub fn valid_path<S: AsRef<str>>(name: S) -> PathBuf {
  let dir = tempdir().expect("Failed to create temp dir");
  let file = name.as_ref();
  dir.path().join(file)
}

pub fn valid_url() -> &'static str {
  // Use a small static file for testing
  "https://httpbin.org/bytes/20"
}

pub fn invalid_url() -> &'static str {
  "ht!tp:::/invalid_url"
}
