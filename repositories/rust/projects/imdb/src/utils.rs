use crate::*;
use std::path::PathBuf;

pub fn inspect_file_manually(path: &std::path::Path) -> Result<()> {
  // let file = File::open(path).map_err(|source| Error::FileRead {
  //   source,
  //   path: PathBuf::from(path),
  //   context: String::from("Failed to open the file to inspect manually"),
  // })?;

  // let file =
  //   File::open(path).map_err(|source| Error::file_open(source, path))?;

  let file =
    File::open(path).context(format!("could not read config at {path:?}"))?;

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
