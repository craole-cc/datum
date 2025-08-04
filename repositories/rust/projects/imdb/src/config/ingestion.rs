// -- Configuration Module (ingestion/config.rs) -- //

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  /// Number of rows to process per chunk
  pub chunk_size: usize,
  /// Maximum number of chunks to prevent runaway processing
  pub max_chunks: usize,
  /// Compression algorithm for Parquet files
  pub compression: ParquetCompressionType,
  /// Whether to use low memory mode during CSV reading
  pub low_memory_mode: bool,
  /// Whether to ignore parsing errors and continue processing
  pub ignore_errors: bool,
  /// Custom null values to recognize during parsing
  pub null_values: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum ParquetCompressionType {
  #[default]
  Snappy,
  Gzip,
  Lz4,
  Brotli,
  Zstd,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      chunk_size: 1_000_000,
      max_chunks: 1000,
      compression: ParquetCompressionType::default(),
      low_memory_mode: false,
      ignore_errors: true,
      null_values: vec![
        "\\N".into(),
        "N/A".into(),
        "N/A.".into(),
        "n/a".into(),
        // "".into(),
      ],
    }
  }
}

impl Config {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
    self.chunk_size = chunk_size;
    self
  }

  pub fn with_compression(
    mut self,
    compression: ParquetCompressionType,
  ) -> Self {
    self.compression = compression;
    self
  }

  pub fn with_max_chunks(mut self, max_chunks: usize) -> Self {
    self.max_chunks = max_chunks;
    self
  }
}
