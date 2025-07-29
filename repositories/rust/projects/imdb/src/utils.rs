use crate::*;

pub fn inspect_file_manually(path: &std::path::Path) -> Result<()> {
  // let file = File::open(path).map_err(|source| {
  //   FilesystemError::file_read_with_ctx(
  //     source,
  //     path,
  //     "Failed to open the file to inspect manually",
  //   )
  // })?;

  let file =
    File::open(path).map_err(|source| Error::fs_file_read(source, path))?;

  // let file = File::open(path).map_err(|source| {
  //   Error::fs_file_read_with_context(
  //     source,
  //     path,
  //     "Failed to open the file to inspect manually",
  //   )
  // })?;
  let reader = BufReader::new(file);
  let mut lines = reader.lines();

  // Read header
  if let Some(Ok(header)) = lines.next() {
    let columns: Vec<&str> = header.split('\t').collect();
    debug!("File has {} columns", columns.len());
    debug!("Column names: {:?}", columns);
  }

  // Count a few more lines to estimate
  let mut count = 1; // header
  for (i, line) in lines.enumerate() {
    if i >= 10 {
      break;
    } // Just check first 10 data rows
    if line.is_ok() {
      count += 1;
    }
  }

  debug!("Successfully read {} lines manually", count);

  Ok(())
}
