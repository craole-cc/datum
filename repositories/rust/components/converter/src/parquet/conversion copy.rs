// -- Conversion Module (ingest/conversion.rs) -- //

use crate::*;

pub fn convert_datasets_to_parquet(datasets: &Datasets) -> Result<()> {
  info!(
    "Converting {} datasets to Parquet format...",
    datasets.iter().count()
  );

  for dataset in datasets.iter() {
    info!("Processing dataset: {}", dataset.name);

    let raw_path = &dataset.files.raw;
    let parquet_path = raw_path.with_extension("parquet");

    // Check if parquet file exists and is newer than the raw file
    if parquet_path.exists() {
      if let (Ok(parquet_meta), Ok(raw_meta)) =
        (metadata(&parquet_path), metadata(raw_path))
      {
        if parquet_meta.modified()? >= raw_meta.modified()? {
          info!("Parquet file is up to date, skipping: {:?}", parquet_path);
          continue;
        }
      } else {
        warn!(
          "Could not compare file timestamps, skipping: {:?}",
          parquet_path
        );
        continue;
      }
    }

    // Check if chunked parquet files already exist and are newer than raw file
    let download_dir = raw_path.parent().ok_or_else(|| {
      anyhow!("Cannot determine parent directory for: {:?}", raw_path)
    })?;

    let existing_chunks: Vec<_> = read_dir(download_dir)
      .context("Failed to read download directory")?
      .filter_map(|entry| entry.ok())
      .map(|entry| entry.path())
      .filter(|path| {
        path
          .file_name()
          .and_then(|name| name.to_str())
          .map(|name| {
            name.starts_with(&format!("{}_", dataset.name))
              && name.ends_with(".parquet")
              && name.contains('_') // Ensure it's a chunk file with underscore
          })
          .unwrap_or(false)
      })
      .collect();

    if !existing_chunks.is_empty() {
      // Check if chunks are newer than raw file
      if let Ok(raw_meta) = metadata(raw_path) {
        let chunks_up_to_date = existing_chunks.iter().all(|chunk_path| {
          metadata(chunk_path)
            .and_then(|chunk_meta| chunk_meta.modified())
            .map(|chunk_time| {
              chunk_time
                >= raw_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH)
            })
            .unwrap_or(false)
        });

        if chunks_up_to_date {
          info!(
            "Chunked parquet files are up to date for {} ({} chunks found)",
            dataset.name,
            existing_chunks.len()
          );
          continue;
        } else {
          warn!(
            "Some chunk files are outdated, recreating all chunks for {}",
            dataset.name
          );
          // Clean up old chunks
          for chunk_path in &existing_chunks {
            if let Err(e) = remove_file(chunk_path) {
              warn!("Failed to remove old chunk {:?}: {}", chunk_path, e);
            }
          }
        }
      }
    }

    // Check if raw TSV file exists
    if !raw_path.exists() {
      error!(
        "Raw file does not exist, skipping {}: {:?}",
        dataset.name, raw_path
      );
      continue;
    }

    info!("Converting TSV to Parquet: {}", dataset.name);
    debug!("Input TSV: {:?}", raw_path);
    debug!("Output Parquet: {:?}", parquet_path);

    // Process the dataset
    if let Err(e) = process_dataset_chunks(dataset, raw_path, &parquet_path) {
      error!("Failed to process dataset {}: {}", dataset.name, e);
      continue;
    }
  }

  info!("All datasets converted to Parquet!");
  Ok(())
}

fn process_dataset_chunks(
  dataset: &Dataset,
  raw_path: &std::path::Path,
  parquet_path: &std::path::Path,
) -> Result<()> {
  info!("Starting chunked processing for: {}", dataset.name);

  let chunk_size = 100_000;
  let mut chunk_number = 0;
  let mut chunk_files = Vec::new();
  let mut total_rows_processed = 0;

  loop {
    debug!("Processing chunk {} for {}", chunk_number, dataset.name);

    let chunk_lf = LazyCsvReader::new(raw_path)
      .with_separator(b'\t')
      .with_has_header(true)
      .with_skip_rows(total_rows_processed)
      .with_n_rows(Some(chunk_size))
      .with_low_memory(true)
      .with_infer_schema_length(Some(0)) // Don't infer schema, treat everything as strings
      .with_ignore_errors(true) // Ignore parsing errors and continue
      .with_null_values(Some(NullValues::AllColumns(vec![
        "\\N".to_string().into(),
        "".to_string().into(), // Also treat empty strings as null
      ])))
      .with_quote_char(None) // Disable quote parsing since TSV shouldn't have quotes
      .with_comment_prefix(None) // Disable comment handling
      .finish()?;

    debug!("Collecting chunk {} data...", chunk_number);
    let chunk_df = match chunk_lf.collect() {
      Ok(df) if df.height() == 0 => {
        debug!("No more data at chunk {}", chunk_number);
        break;
      }
      Ok(df) => {
        trace!("Collected {} rows for chunk {}", df.height(), chunk_number);
        df
      }
      Err(e) => {
        warn!(
          "Error collecting chunk {} for {}: {}",
          chunk_number, dataset.name, e
        );
        warn!("Attempting to continue with next chunk...");
        total_rows_processed += chunk_size; // Skip this chunk
        chunk_number += 1;
        continue;
      }
    };

    let chunk_path = parquet_path
      .with_file_name(format!("{}_{:06}.parquet", dataset.name, chunk_number));

    debug!("Writing chunk {} to {:?}", chunk_number, chunk_path);
    let mut chunk_file = File::create(&chunk_path).with_context(|| {
      format!("Failed to create chunk file: {chunk_path:?}")
    })?;

    match ParquetWriter::new(&mut chunk_file)
      .with_compression(ParquetCompression::Snappy)
      .with_statistics(StatisticsOptions::default())
      .finish(&mut chunk_df.clone())
    {
      Ok(_) => {
        chunk_files.push(chunk_path);
        total_rows_processed += chunk_df.height();
        chunk_number += 1;

        debug!(
          "✅ Chunk {} complete ({} rows, {} total processed)",
          chunk_number - 1,
          chunk_df.height(),
          total_rows_processed
        );

        if chunk_df.height() < chunk_size {
          debug!("Last chunk processed (incomplete chunk size)");
          break;
        }
      }
      Err(e) => {
        error!(
          "Failed to write chunk {} for {}: {}",
          chunk_number, dataset.name, e
        );
        // Remove the failed file and continue
        if let Err(remove_err) = remove_file(&chunk_path) {
          warn!(
            "Failed to remove failed chunk file {:?}: {}",
            chunk_path, remove_err
          );
        }
        total_rows_processed += chunk_df.height();
        chunk_number += 1;
        continue;
      }
    }

    // Safety check to prevent infinite loops
    if chunk_number > 1000 {
      error!(
        "Too many chunks ({}), stopping to prevent runaway processing",
        chunk_number
      );
      break;
    }
  }

  if chunk_files.is_empty() {
    return Err(anyhow!("No chunk files were created for {}", dataset.name));
  }

  info!(
    "Successfully created {} chunk files for {}",
    chunk_files.len(),
    dataset.name
  );

  // Verify the chunk files were actually created and log size info
  let existing_chunks: Vec<_> =
    chunk_files.iter().filter(|p| p.exists()).collect();

  if !existing_chunks.is_empty() {
    // Calculate total size of chunks
    let total_size: u64 = existing_chunks
      .iter()
      .filter_map(|path| metadata(path).ok())
      .map(|meta| meta.len())
      .sum();

    if let Ok(raw_meta) = metadata(raw_path) {
      let raw_size = raw_meta.len();
      let compression_ratio = raw_size as f64 / total_size as f64;
      info!(
        "{} compression: {:.1}MB -> {:.1}MB ({:.1}x reduction) across {} files",
        dataset.name,
        raw_size as f64 / 1_048_576.0,
        total_size as f64 / 1_048_576.0,
        compression_ratio,
        existing_chunks.len()
      );
    }
  } else {
    return Err(anyhow!(
      "No chunk files were successfully created for {}",
      dataset.name
    ));
  }

  Ok(())
}

pub fn load_parquet_dataset(dataset: &Dataset) -> Result<LazyFrame> {
  // First check if we have a single parquet file
  let single_parquet_path = dataset.files.raw.with_extension("parquet");

  if single_parquet_path.exists() {
    debug!(
      "Loading {} from single Parquet: {:?}",
      dataset.name, single_parquet_path
    );
    return LazyFrame::scan_parquet(
      single_parquet_path,
      ScanArgsParquet::default(),
    )
    .with_context(|| {
      format!("Failed to load single Parquet for {}", dataset.name)
    });
  }

  // Look for chunked parquet files
  let download_dir = dataset
    .files
    .raw
    .parent()
    .ok_or_else(|| anyhow!("Cannot get parent directory of raw file"))?;

  let pattern = format!("{}_*.parquet", dataset.name);
  let mut chunk_files: Vec<_> = read_dir(download_dir)
    .context("Failed to read download directory")?
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.path())
    .filter(|path| {
      path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| {
          name.starts_with(&format!("{}_", dataset.name))
            && name.ends_with(".parquet")
        })
        .unwrap_or(false)
    })
    .collect();

  if chunk_files.is_empty() {
    return Err(anyhow!(
      "No Parquet files found for {}: neither single file {:?} nor chunks matching {}",
      dataset.name,
      single_parquet_path,
      pattern
    ));
  }

  // Sort chunk files to ensure proper order
  chunk_files.sort();

  info!(
    "Loading {} from {} chunk files",
    dataset.name,
    chunk_files.len()
  );

  // Read all chunks and concatenate them
  let lazy_frames: Result<Vec<LazyFrame>> = chunk_files
    .iter()
    .map(|path| {
      LazyFrame::scan_parquet(path.clone(), ScanArgsParquet::default())
        .with_context(|| format!("Failed to load chunk: {path:?}"))
    })
    .collect();

  let frames = lazy_frames?;

  if frames.is_empty() {
    return Err(anyhow!(
      "No valid parquet chunks found for {}",
      dataset.name
    ));
  }

  // Concatenate all chunks into a single LazyFrame using polars concat function
  let combined = concat(&frames, UnionArgs::default()).with_context(|| {
    format!("Failed to concatenate chunks for {}", dataset.name)
  })?;

  trace!(
    "Successfully loaded and concatenated {} chunks",
    frames.len()
  );
  Ok(combined)
}

// Alternative: Load a specific chunk
pub fn load_parquet_chunk(
  dataset: &Dataset,
  chunk_index: usize,
) -> Result<LazyFrame> {
  let chunk_path = dataset
    .files
    .raw
    .parent()
    .ok_or_else(|| anyhow!("Cannot get parent directory"))?
    .join(format!("{}_{:06}.parquet", dataset.name, chunk_index));

  if !chunk_path.exists() {
    return Err(anyhow!(
      "Chunk {} not found for {}: {:?}",
      chunk_index,
      dataset.name,
      chunk_path
    ));
  }

  debug!(
    "Loading chunk {} for {}: {:?}",
    chunk_index, dataset.name, chunk_path
  );

  LazyFrame::scan_parquet(chunk_path, ScanArgsParquet::default()).with_context(
    || format!("Failed to load chunk {} for {}", chunk_index, dataset.name),
  )
}

// Get chunk count for a dataset
pub fn get_chunk_count(dataset: &Dataset) -> usize {
  let download_dir = match dataset.files.raw.parent() {
    Some(dir) => dir,
    None => return 0,
  };

  read_dir(download_dir)
    .map(|entries| {
      entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
          entry
            .file_name()
            .to_str()
            .map(|name| {
              name.starts_with(&format!("{}_", dataset.name))
                && name.ends_with(".parquet")
            })
            .unwrap_or(false)
        })
        .count()
    })
    .unwrap_or(0)
}
