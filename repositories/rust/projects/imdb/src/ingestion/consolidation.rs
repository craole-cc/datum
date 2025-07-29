// -- Consolidation Operations Module (ingestion/consolidation.rs) -- //

use super::*;

pub struct Processor {
  config: Config,
}

impl Processor {
  pub fn new(config: Config) -> Self {
    Self { config }
  }

  pub fn consolidate_chunks(
    &self,
    dataset: &Dataset,
    paths: &Paths,
  ) -> Result<()> {
    info!("Consolidating chunks for dataset: {}", dataset.name);

    let chunk_paths = paths.list_existing_chunks()?;
    if chunk_paths.is_empty() {
      return Err(
        ValidationError::empty_collection("chunks for consolidation").into(),
      );
    }

    debug!("Found {} chunks to consolidate", chunk_paths.len());

    let consolidated_df = self.load_and_combine_chunks(&chunk_paths)?;
    self
      .write_consolidated_parquet(&consolidated_df, &paths.consolidated_file)?;

    info!(
      "Successfully consolidated {} chunks into: {:?}",
      chunk_paths.len(),
      paths.consolidated_file
    );

    Ok(())
  }

  fn load_and_combine_chunks(
    &self,
    chunk_paths: &[std::path::PathBuf],
  ) -> Result<LazyFrame> {
    debug!("Loading {} chunks for consolidation", chunk_paths.len());

    let lazy_frames: Result<Vec<LazyFrame>> = chunk_paths
      .iter()
      .map(|path| {
        LazyFrame::scan_parquet(path.clone(), ScanArgsParquet::default())
          .map_err(|e| {
            ValidationError::invalid_format(
              "parquet_chunk",
              "valid parquet file",
              &format!("failed to scan {}: {}", path.display(), e),
            )
            .into()
          })
      })
      .collect();

    let frames = lazy_frames?;

    if frames.is_empty() {
      return Err(
        ValidationError::empty_collection("valid parquet chunks").into(),
      );
    }

    // Use polars concat function to combine all chunks
    let combined = concat(&frames, UnionArgs::default())
      .context("Failed to concatenate chunks during consolidation")?;

    trace!("Successfully combined {} chunks", frames.len());
    Ok(combined)
  }

  fn write_consolidated_parquet(
    &self,
    lazy_frame: &LazyFrame,
    output_path: &std::path::Path,
  ) -> Result<()> {
    debug!("Writing consolidated parquet to: {:?}", output_path);

    // Collect the lazy frame to get a concrete DataFrame
    let df = lazy_frame.clone().collect().map_err(|e| {
      ValidationError::invalid_format(
        "lazy_frame",
        "collectible dataframe",
        &format!("consolidation failed: {}", e),
      )
    })?;

    let mut output_file = File::create(output_path).map_err(|e| {
      e.file_create(output_path, Some("creating consolidated file"))
    })?;

    ParquetWriter::new(&mut output_file)
      .with_compression(self.config.to_polars_compression())
      .with_statistics(self.config.statistics)
      .finish(&mut df.clone())
      .context({
        format!("Failed to write consolidated parquet: {output_path:?}")
      })?;

    debug!(
      "Consolidated parquet written successfully with {} rows",
      df.height()
    );
    Ok(())
  }

  pub fn load_consolidated_dataset(&self, paths: &Paths) -> Result<LazyFrame> {
    if paths.consolidated_file.exists() {
      debug!(
        "Loading consolidated parquet: {:?}",
        paths.consolidated_file
      );
      return LazyFrame::scan_parquet(
        paths.consolidated_file.clone(),
        ScanArgsParquet::default(),
      )
      .map_err(|e| {
        ValidationError::invalid_format(
          "consolidated_parquet",
          "valid parquet file",
          &format!(
            "failed to scan {}: {}",
            paths.consolidated_file.display(),
            e
          ),
        )
        .into()
      });
    }

    // Fall back to loading from chunks
    warn!("Consolidated file not found, loading from chunks");
    self.load_from_chunks(paths)
  }

  pub fn load_from_chunks(&self, paths: &Paths) -> Result<LazyFrame> {
    let chunk_paths = paths.list_existing_chunks()?;
    if chunk_paths.is_empty() {
      return Err(ValidationError::context(format!(
            "No parquet files found: neither consolidated file '{}' nor chunks exist",
            paths.consolidated_file.display()
        )).into());
    }

    info!("Loading dataset from {} chunk files", chunk_paths.len());
    self.load_and_combine_chunks(&chunk_paths)
  }

  pub fn load_specific_chunk(
    &self,
    paths: &Paths,
    chunk_index: usize,
  ) -> Result<LazyFrame> {
    let chunk_path = paths.get_chunk_path(chunk_index);

    if !chunk_path.exists() {
      return Err(
        ValidationError::item_not_found(
          "chunks",
          &format!("chunk {} at {}", chunk_index, chunk_path.display()),
        )
        .into(),
      );
    }

    debug!("Loading specific chunk {}: {:?}", chunk_index, chunk_path);

    LazyFrame::scan_parquet(chunk_path.clone(), ScanArgsParquet::default())
      .map_err(|e| {
        ValidationError::invalid_format(
          "parquet_chunk",
          "valid parquet file",
          &format!("failed to load chunk {}: {}", chunk_index, e),
        )
        .into()
      })
  }

  pub fn validate_chunk_count(
    &self,
    paths: &Paths,
    expected_min: usize,
  ) -> Result<()> {
    let chunk_paths = paths.list_existing_chunks()?;
    if chunk_paths.len() < expected_min {
      return Err(
        ValidationError::constraint_violation(format!(
          "Expected at least {} chunks, found {}",
          expected_min,
          chunk_paths.len()
        ))
        .into(),
      );
    }
    Ok(())
  }
}
