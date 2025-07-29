// -- Validation Operations Module (ingestion/validation.rs) -- //

use super::*;

pub struct Processor;

impl Processor {
  pub fn new() -> Self {
    Self
  }

  pub fn should_process_dataset(
    &self,
    dataset: &Dataset,
    paths: &Paths,
  ) -> Result<Kind> {
    // Check if raw file exists
    if !paths.raw_file.exists() {
      return Ok(Kind::Skip(format!(
        "Raw file does not exist: {:?}",
        paths.raw_file
      )));
    }

    // Check consolidated file first
    if paths.is_consolidated_up_to_date()? {
      return Ok(Kind::Skip(
        "Consolidated parquet file is up to date".to_string(),
      ));
    }

    // Check if chunks are up to date
    if paths.are_chunks_up_to_date()? {
      let chunks = paths.list_existing_chunks()?;
      if !chunks.is_empty() {
        return Ok(Kind::ConsolidateOnly(format!(
          "Chunks are up to date ({} chunks found), only consolidation needed",
          chunks.len()
        )));
      }
    }

    // Need full processing
    Ok(Kind::FullProcess)
  }

  pub fn validate_processing_result(
    &self,
    dataset: &Dataset,
    paths: &Paths,
  ) -> Result<Outcome> {
    let mut issues = Vec::new();
    let mut warnings = Vec::new();

    // Check if chunks were created
    let chunks = paths.list_existing_chunks()?;
    if chunks.is_empty() {
      issues.push("No chunk files were created".to_string());
    } else {
      debug!("Found {} chunk files", chunks.len());
    }

    // Check if consolidated file was created
    if !paths.consolidated_file.exists() {
      issues.push("Consolidated parquet file was not created".to_string());
    } else {
      debug!("Consolidated file exists: {:?}", paths.consolidated_file);
    }

    // Validate file timestamps
    if let Err(e) = self.validate_timestamps(paths) {
      warnings.push(format!("Timestamp validation failed: {e}"));
    }

    // Get compression statistics
    let stats = paths.get_compression_stats().unwrap_or_else(|e| {
      warnings.push(format!("Could not get compression stats: {e}"));
      super::paths::CompressionStats {
        raw_size: 0,
        chunks_total_size: 0,
        consolidated_size: None,
        chunk_count: 0,
      }
    });

    let result = if issues.is_empty() {
      Outcome::Success { stats, warnings }
    } else {
      Outcome::Failed { issues, warnings }
    };

    Ok(result)
  }

  fn validate_timestamps(&self, paths: &Paths) -> Result<()> {
    // Get raw file metadata and modification time
    let raw_meta = std::fs::metadata(&paths.raw_file).map_err(|e| {
      e.file_read(&paths.raw_file, Some("validating raw file timestamp"))
    })?;
    let raw_modified = raw_meta.modified().map_err(|e| {
      e.file_read(&paths.raw_file, Some("getting raw file modification time"))
    })?;

    // Check chunk timestamps
    let chunks = paths.list_existing_chunks()?;
    for chunk_path in &chunks {
      let chunk_meta = std::fs::metadata(chunk_path).map_err(|e| {
        e.file_read(chunk_path, Some("validating chunk timestamp"))
      })?;
      let chunk_modified = chunk_meta.modified().map_err(|e| {
        e.file_read(chunk_path, Some("getting chunk modification time"))
      })?;

      if chunk_modified < raw_modified {
        return Err(
          ValidationError::constraint_violation(format!(
            "Chunk '{}' is older than raw file",
            chunk_path.display()
          ))
          .into(),
        );
      }
    }

    // Check consolidated file timestamp
    if paths.consolidated_file.exists() {
      let consolidated_meta = std::fs::metadata(&paths.consolidated_file)
        .map_err(|e| {
          e.file_read(
            &paths.consolidated_file,
            Some("validating consolidated file timestamp"),
          )
        })?;
      let consolidated_modified =
        consolidated_meta.modified().map_err(|e| {
          e.file_read(
            &paths.consolidated_file,
            Some("getting consolidated file modification time"),
          )
        })?;

      if consolidated_modified < raw_modified {
        return Err(
          ValidationError::constraint_violation(
            "Consolidated file is older than raw file",
          )
          .into(),
        );
      }
    }

    Ok(())
  }

  pub fn log_processing_decision(&self, dataset_name: &str, decision: &Kind) {
    match decision {
      Kind::Skip(reason) => {
        info!("Skipping {}: {}", dataset_name, reason);
      }
      Kind::ConsolidateOnly(reason) => {
        info!("Consolidation only for {}: {}", dataset_name, reason);
      }
      Kind::FullProcess => {
        info!("Full processing required for {}", dataset_name);
      }
    }
  }

  pub fn log_validation_result(&self, dataset_name: &str, result: &Outcome) {
    match result {
      Outcome::Success { stats, warnings } => {
        info!("✅ Processing completed successfully for {}", dataset_name);
        stats.log_stats(dataset_name);

        for warning in warnings {
          warn!("{}: {}", dataset_name, warning);
        }
      }
      Outcome::Failed { issues, warnings } => {
        error!("❌ Processing failed for {}", dataset_name);

        for issue in issues {
          error!("{}: {}", dataset_name, issue);
        }

        for warning in warnings {
          warn!("{}: {}", dataset_name, warning);
        }
      }
    }
  }
}

#[derive(Debug)]
pub enum Kind {
  Skip(String),
  ConsolidateOnly(String),
  FullProcess,
}

#[derive(Debug)]
pub enum Outcome {
  Success {
    stats: super::paths::CompressionStats,
    warnings: Vec<String>,
  },
  Failed {
    issues: Vec<String>,
    warnings: Vec<String>,
  },
}
