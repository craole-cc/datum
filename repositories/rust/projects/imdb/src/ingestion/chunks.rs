// -- Chunking Operations Module (ingestion/chunks.rs) -- //

use super::*;

pub struct Processor {
  config: Config,
}

impl Processor {
  pub fn new(config: Config) -> Self {
    Self { config }
  }

  pub fn process_dataset_to_chunks(
    &self,
    dataset: &Dataset,
    paths: &Paths,
  ) -> Result<ChunkingResult> {
    info!("Starting chunked processing for: {}", dataset.name);

    paths.ensure_chunks_directory()?;

    let mut chunk_number = 0;
    let mut chunk_files = Vec::new();
    let mut total_rows_processed = 0;
    let mut skip_rows = 0;

    loop {
      debug!("Processing chunk {} for {}", chunk_number, dataset.name);

      let chunk_result =
        self.process_single_chunk(dataset, paths, chunk_number, skip_rows)?;

      match chunk_result {
        Outcome::Success {
          rows_processed,
          chunk_path,
        } => {
          chunk_files.push(chunk_path);
          total_rows_processed += rows_processed;
          skip_rows += rows_processed;
          chunk_number += 1;

          debug!(
            "✅ Chunk {} complete ({} rows, {} total processed)",
            chunk_number - 1,
            rows_processed,
            total_rows_processed
          );

          // If we got fewer rows than chunk_size, we're done
          if rows_processed < self.config.chunk_size {
            debug!("Last chunk processed (incomplete chunk size)");
            break;
          }
        }
        Outcome::Empty => {
          debug!("No more data at chunk {}", chunk_number);
          break;
        }
        Outcome::Error(e) => {
          warn!(
            "Error processing chunk {} for {}: {}",
            chunk_number, dataset.name, e
          );
          warn!("Attempting to continue with next chunk...");
          skip_rows += self.config.chunk_size; // Skip this chunk
          chunk_number += 1;
          continue;
        }
      }

      // Safety check to prevent infinite loops
      if chunk_number >= self.config.max_chunks {
        error!(
          "Reached maximum chunks ({}), stopping to prevent runaway processing",
          self.config.max_chunks
        );
        break;
      }
    }

    if chunk_files.is_empty() {
      return Err(ValidationError::empty_collection("chunk_files").into());
    }

    info!(
      "Successfully created {} chunk files for {}",
      chunk_files.len(),
      dataset.name
    );

    Ok(ChunkingResult {
      chunk_files,
      total_rows_processed,
    })
  }

  fn process_single_chunk(
    &self,
    dataset: &Dataset,
    paths: &Paths,
    chunk_number: usize,
    skip_rows: usize,
  ) -> Result<Outcome> {
    let chunk_lf = self.create_chunk_lazy_frame(&paths.raw_file, skip_rows)?;

    debug!("Collecting chunk {} data...", chunk_number);
    let chunk_df = match chunk_lf.collect() {
      Ok(df) if df.height() == 0 => {
        return Ok(Outcome::Empty);
      }
      Ok(df) => {
        trace!("Collected {} rows for chunk {}", df.height(), chunk_number);
        df
      }
      Err(e) => {
        return Ok(Outcome::Error(
          ValidationError::context(format!(
            "Failed to collect data for chunk {}: {}",
            chunk_number, e
          ))
          .into(),
        ));
      }
    };

    let chunk_path = paths.get_chunk_path(chunk_number);
    self.write_chunk_to_parquet(&chunk_df, &chunk_path)?;

    Ok(Outcome::Success {
      rows_processed: chunk_df.height(),
      chunk_path,
    })
  }

  fn create_chunk_lazy_frame(
    &self,
    raw_path: &Path,
    skip_rows: usize,
  ) -> Result<LazyFrame> {
    let lazy_frame = LazyCsvReader::new(raw_path)
      .with_separator(b'\t')
      .with_has_header(skip_rows == 0) // Only first chunk has header
      .with_skip_rows(if skip_rows == 0 { 0 } else { skip_rows + 1 }) // +1 to skip header on subsequent chunks
      .with_n_rows(Some(self.config.chunk_size))
      .with_low_memory(self.config.low_memory_mode)
      .with_infer_schema_length(Some(0)) // Treat everything as strings
      .with_ignore_errors(self.config.ignore_errors)
      .with_null_values(Some(self.config.get_null_values()))
      .with_quote_char(None) // Disable quote parsing for TSV
      .with_comment_prefix(None) // Disable comment handling
      .finish()
      .map_err(|e| {
        ValidationError::invalid_format(
          "csv_reader",
          "valid TSV file",
          &format!("polars error: {}", e),
        )
      })?;

    Ok(lazy_frame)
  }

  fn write_chunk_to_parquet(
    &self,
    chunk_df: &DataFrame,
    chunk_path: &Path,
  ) -> Result<()> {
    debug!("Writing chunk to {:?}", chunk_path);

    let mut chunk_file = File::create(chunk_path)
      .map_err(|e| e.file_create(chunk_path, Some("creating chunk file")))?;

    let statistics = self.config.statistics;

    ParquetWriter::new(&mut chunk_file)
      .with_compression(self.config.to_polars_compression())
      .with_statistics(statistics)
      .finish(&mut chunk_df.clone())
      .map_err(|e| {
        ValidationError::invalid_format(
          "parquet_writer",
          "valid parquet file",
          &format!("failed to write to {}: {}", chunk_path.display(), e),
        )
      })?;

    Ok(())
  }
}

#[derive(Debug)]
pub struct ChunkingResult {
  pub chunk_files: Vec<PathBuf>,
  pub total_rows_processed: usize,
}

#[derive(Debug)]
enum Outcome {
  Success {
    rows_processed: usize,
    chunk_path: PathBuf,
  },
  Empty,
  Error(crate::Error), // Changed from anyhow::Error to your Error type
}
