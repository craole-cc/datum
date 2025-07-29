// -- Parquet Conversion Module (parquet/conversion.rs) -- //

use super::*;

// entry point
pub fn from_tsv_datasets(datasets: &Datasets) -> Result<()> {
  info!("Converting {} datasets...", datasets.len());
  for dataset in datasets.iter() {
    process_single_dataset(dataset)?;
  }
  info!("All datasets converted to Parquet!");
  Ok(())
}

// One function = one dataset lifecycle
fn process_single_dataset(dataset: &Dataset) -> Result<()> {
  if !dataset.files.raw.exists() {
    error!("Raw file not found: {:?}", &dataset.files.raw);
    return Ok(());
  }
  if parquet_up_to_date(dataset)? || chunks_up_to_date(dataset)? {
    return Ok(());
  }
  clean_stale_chunks(dataset)?; // does nothing if none to clean
  convert_tsv_to_parquet_chunks(dataset)?;
  Ok(())
}

// Returns true if the monolithic Parquet is up to date
fn parquet_up_to_date(dataset: &Dataset) -> Result<bool> {
  let pqt = dataset.files.raw.with_extension("parquet");
  Ok(pqt.exists() && file_newer(&pqt, &dataset.files.raw)?)
}

// Returns true if all chunked Parquet files are up to date
fn chunks_up_to_date(dataset: &Dataset) -> Result<bool> {
  let chunks = find_parquet_chunks(dataset)?;
  if chunks.is_empty() {
    return Ok(false);
  }
  for chunk in &chunks {
    if !file_newer(chunk, &dataset.files.raw)? {
      return Ok(false);
    }
  }
  info!("Chunked parquet files up-to-date ({})", chunks.len());
  Ok(true)
}

// Removes all chunk files for the dataset (for cleanup/reset)
fn clean_stale_chunks(dataset: &Dataset) -> Result<()> {
  for p in find_parquet_chunks(dataset)? {
    if let Err(e) = remove_file(&p) {
      warn!("Could not remove old chunk {:?}: {e}", p);
    }
  }
  Ok(())
}

// Finds all chunk parquet files for a dataset
fn find_parquet_chunks(dataset: &Dataset) -> Result<Vec<PathBuf>> {
  let dir = dataset
    .files
    .raw
    .parent()
    .ok_or_else(|| anyhow!("No parent dir"))?;
  let pattern = format!("{}_", dataset.name);
  Ok(
    read_dir(dir)?
      .filter_map(|e| e.ok().map(|e| e.path()))
      .filter(|p| {
        p.file_name()
          .and_then(|n| n.to_str())
          .map(|n| n.starts_with(&pattern) && n.ends_with(".parquet"))
          .unwrap_or(false)
      })
      .collect(),
  )
}

// Checks if 'file_a' is newer or same as 'file_b'
fn file_newer(file_a: &Path, file_b: &Path) -> Result<bool> {
  let ma = metadata(file_a)?.modified()?;
  let mb = metadata(file_b)?.modified()?;
  Ok(ma >= mb)
}

// Converts TSV to Parquet chunks in a loop, using provided chunk_size
fn convert_tsv_to_parquet_chunks(dataset: &Dataset) -> Result<()> {
  let chunk_size = 100_000; // or make configurable
  let mut row_cursor = 0;
  let mut chunk_no = 0;

  loop {
    let chunk_df = load_tsv_chunk(dataset, row_cursor, chunk_size)?;
    if chunk_df.height() == 0 {
      break;
    }
    write_parquet_chunk(dataset, chunk_no, &chunk_df)?;
    row_cursor += chunk_df.height();
    chunk_no += 1;
    if chunk_df.height() < chunk_size {
      break;
    }
    if chunk_no > 1000 {
      error!("Excessive chunks; aborting");
      break;
    }
  }
  Ok(())
}

// Loads a chunk of TSV as a DataFrame
fn load_tsv_chunk(
  dataset: &Dataset,
  skip: usize,
  chunk_size: usize,
) -> Result<DataFrame> {
  // Polars: use LazyCsvReader here as before, all your error/context flags
  // Return Ok(df)
}

// Writes DataFrame as a chunked Parquet file
fn write_parquet_chunk(
  dataset: &Dataset,
  chunk_no: usize,
  df: &DataFrame,
) -> Result<()> {
  let out_path = dataset
    .files
    .raw
    .parent()
    .ok_or_else(|| anyhow!("No parent dir"))?
    .join(format!("{}_{:06}.parquet", dataset.name, chunk_no));
  // Use Polars ParquetWriter exactly as before, but all logic in one function
  // Error handling here ensures files aren't left partially written
  Ok(())
}
