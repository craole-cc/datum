// -- Main Orchestrator Module (ingest/orchestrator.rs) -- //

// region: Imports
// use super::{Error, Results, paths,chunks,consolidation,validation};
use super::*;
// endregion
// region: Structure
pub struct ConversionOrchestrator {
  config: Config,
  chunk_processor: ChunkProcessor,
  consolidation_processor: ConsolidationProcessor,
  validation_processor: ValidationProcessor,
}

impl ConversionOrchestrator {
  pub fn new(config: Config) -> Self {
    let chunk_processor = ChunkProcessor::new(config.clone());
    let consolidation_processor = ConsolidationProcessor::new(config.clone());
    let validation_processor = ValidationProcessor::new();

    Self {
      config,
      chunk_processor,
      consolidation_processor,
      validation_processor,
    }
  }

  pub fn with_default_config() -> Self {
    Self::new(Config::default())
  }

  pub fn convert_datasets_to_parquet(
    &self,
    datasets: &Datasets,
  ) -> Result<ConversionSummary> {
    info!(
      "Converting {} datasets to Parquet format...",
      datasets.iter().count()
    );

    let mut summary = ConversionSummary::new();

    for dataset in datasets.iter() {
      info!("Processing dataset: {}", dataset.name);

      match self.process_single_dataset(dataset) {
        Ok(result) => {
          summary.add_success(dataset.name.clone(), result);
        }
        Err(e) => {
          error!("Failed to process dataset {}: {}", dataset.name, e);
          summary.add_failure(dataset.name.clone(), e);
        }
      }
    }

    info!("Dataset conversion completed!");
    summary.log_summary();
    Ok(summary)
  }

  fn process_single_dataset(
    &self,
    dataset: &Dataset,
  ) -> Result<DatasetProcessingResult> {
    let paths = paths::Dataset::new(dataset)?;

    // Determine what processing is needed
    let decision = self
      .validation_processor
      .should_process_dataset(dataset, &paths)?;
    self
      .validation_processor
      .log_processing_decision(&dataset.name, &decision);

    let processing_result = match decision {
      ProcessingDecision::Skip(_) => DatasetProcessingResult::Skipped,
      ProcessingDecision::ConsolidateOnly(_) => {
        self
          .consolidation_processor
          .consolidate_chunks(dataset, &paths)?;
        DatasetProcessingResult::ConsolidatedOnly
      }
      ProcessingDecision::FullProcess => {
        // Clean up old chunks if they exist
        if paths.chunks_dir.exists() {
          paths.clean_old_chunks()?;
        }

        // Process chunks
        let chunking_result = self
          .chunk_processor
          .process_dataset_to_chunks(dataset, &paths)?;

        // Consolidate chunks
        self
          .consolidation_processor
          .consolidate_chunks(dataset, &paths)?;

        DatasetProcessingResult::FullyProcessed {
          chunks_created: chunking_result.chunk_files.len(),
          total_rows: chunking_result.total_rows_processed,
        }
      }
    };

    // Validate the result
    let validation_result = self
      .validation_processor
      .validate_processing_result(dataset, &paths)?;
    self
      .validation_processor
      .log_validation_result(&dataset.name, &validation_result);

    match validation_result {
      ValidationResult::Success { .. } => Ok(processing_result),
      ValidationResult::Failed { issues, .. } => {
        Err(anyhow!("Validation failed: {}", issues.join("; ")))
      }
    }
  }

  // Public API methods for loading data
  pub fn load_dataset(&self, dataset: &Dataset) -> Result<LazyFrame> {
    let paths = DatasetPaths::new(dataset)?;
    self
      .consolidation_processor
      .load_consolidated_dataset(&paths)
  }

  pub fn load_dataset_from_chunks(
    &self,
    dataset: &Dataset,
  ) -> Result<LazyFrame> {
    let paths = DatasetPaths::new(dataset)?;
    self.consolidation_processor.load_from_chunks(&paths)
  }

  pub fn load_specific_chunk(
    &self,
    dataset: &Dataset,
    chunk_index: usize,
  ) -> Result<LazyFrame> {
    let paths = DatasetPaths::new(dataset)?;
    self
      .consolidation_processor
      .load_specific_chunk(&paths, chunk_index)
  }

  pub fn get_chunk_count(&self, dataset: &Dataset) -> Result<usize> {
    let paths = DatasetPaths::new(dataset)?;
    Ok(paths.list_existing_chunks()?.len())
  }

  pub fn get_dataset_stats(
    &self,
    dataset: &Dataset,
  ) -> Result<super::paths::CompressionStats> {
    let paths = DatasetPaths::new(dataset)?;
    paths.get_compression_stats()
  }
}

#[derive(Debug)]
pub struct ConversionSummary {
  pub successful: Vec<(String, DatasetProcessingResult)>,
  pub failed: Vec<(String, anyhow::Error)>,
}

impl ConversionSummary {
  fn new() -> Self {
    Self {
      successful: Vec::new(),
      failed: Vec::new(),
    }
  }

  fn add_success(
    &mut self,
    dataset_name: String,
    result: DatasetProcessingResult,
  ) {
    self.successful.push((dataset_name, result));
  }

  fn add_failure(&mut self, dataset_name: String, error: anyhow::Error) {
    self.failed.push((dataset_name, error));
  }

  pub fn is_successful(&self) -> bool {
    self.failed.is_empty()
  }

  pub fn total_datasets(&self) -> usize {
    self.successful.len() + self.failed.len()
  }

  fn log_summary(&self) {
    let total = self.total_datasets();
    let successful = self.successful.len();
    let failed = self.failed.len();

    info!(
      "Conversion Summary: {}/{} datasets processed successfully",
      successful, total
    );

    if failed > 0 {
      warn!("Failed datasets ({}):", failed);
      for (name, error) in &self.failed {
        warn!("  - {}: {}", name, error);
      }
    }

    if successful > 0 {
      info!("Successful datasets ({}):", successful);
      for (name, result) in &self.successful {
        match result {
          DatasetProcessingResult::Skipped => {
            info!("  - {} (skipped - up to date)", name);
          }
          DatasetProcessingResult::ConsolidatedOnly => {
            info!("  - {} (consolidated only)", name);
          }
          DatasetProcessingResult::FullyProcessed {
            chunks_created,
            total_rows,
          } => {
            info!(
              "  - {} ({} chunks, {} rows)",
              name, chunks_created, total_rows
            );
          }
        }
      }
    }
  }
}

#[derive(Debug, Clone)]
pub enum DatasetProcessingResult {
  Skipped,
  ConsolidatedOnly,
  FullyProcessed {
    chunks_created: usize,
    total_rows: usize,
  },
}

// endregion
// region: Workflow Convenience Functions
pub fn convert_datasets_to_parquet(
  datasets: &Datasets,
) -> Result<ConversionSummary> {
  let orchestrator = ConversionOrchestrator::with_default_config();
  orchestrator.convert_datasets_to_parquet(datasets)
}

pub fn convert_datasets_with_config(
  datasets: &Datasets,
  config: Config,
) -> Result<ConversionSummary> {
  let orchestrator = ConversionOrchestrator::new(config);
  orchestrator.convert_datasets_to_parquet(datasets)
}

pub fn load_parquet_dataset(dataset: &Dataset) -> Result<LazyFrame> {
  let orchestrator = ConversionOrchestrator::with_default_config();
  orchestrator.load_dataset(dataset)
}

pub fn load_parquet_chunk(
  dataset: &Dataset,
  chunk_index: usize,
) -> Result<LazyFrame> {
  let orchestrator = ConversionOrchestrator::with_default_config();
  orchestrator.load_specific_chunk(dataset, chunk_index)
}

pub fn get_chunk_count(dataset: &Dataset) -> Result<usize> {
  let orchestrator = ConversionOrchestrator::with_default_config();
  orchestrator.get_chunk_count(dataset)
}
// endregion
