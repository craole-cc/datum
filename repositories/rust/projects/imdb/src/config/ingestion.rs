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
        "".into(),
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
pub fn assume_delimiter(path: &Path) -> Result<u8> {
  let result = || -> Result<u8> {
    let file_labels = [miette::LabeledSpan::at(
      0..path.to_string_lossy().len(),
      "file path",
    )];
    let file = enriched_error!(
      File::open(path),
      "Failed to open file for delimiter detection",
      code = "FILE_OPEN_ERROR",
      help = "Ensure the file exists and you have read permissions. Check the file path is correct.",
      severity = miette::Severity::Error,
      labels = file_labels
    )?;

    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    let read_labels = [miette::LabeledSpan::at(0..0, "first line")];
    enriched_error!(
      reader.read_line(&mut first_line),
      "Failed to read first line for delimiter detection",
      code = "FILE_READ_ERROR",
      help = "File may be empty, corrupted, or contain invalid UTF-8. Try opening the file in a text editor to verify its contents.",
      severity = miette::Severity::Error,
      labels = read_labels
    )?;

    // Check if file is empty
    if first_line.trim().is_empty() {
      return Err(miette::miette!(
        code = "EMPTY_FILE_ERROR",
        help =
          "Provide a file with at least one line of data to detect delimiters",
        severity = miette::Severity::Warning,
        labels = vec![miette::LabeledSpan::at(0..0, "empty content")],
        "Cannot detect delimiter: file appears to be empty"
      ));
    }

    // Count potential delimiters in first line
    let comma_count = first_line.matches(',').count();
    let tab_count = first_line.matches('\t').count();
    let pipe_count = first_line.matches('|').count();
    let semicolon_count = first_line.matches(';').count();

    // Log what we found for debugging
    debug!(
      "Delimiter analysis: commas={}, tabs={}, pipes={}, semicolons={}",
      comma_count, tab_count, pipe_count, semicolon_count
    );

    match (comma_count, tab_count, pipe_count, semicolon_count) {
      (c, t, p, s) if c > t && c > p && c > s => {
        info!("Detected CSV format (comma-separated)");
        Ok(b',')
      }
      (c, t, p, s) if t > c && t > p && t > s => {
        info!("Detected TSV format (tab-separated)");
        Ok(b'\t')
      }
      (c, t, p, s) if p > c && p > t && p > s => {
        info!("Detected pipe-separated format");
        Ok(b'|')
      }
      (c, t, p, s) if s > c && s > t && s > p => {
        info!("Detected semicolon-separated format");
        Ok(b';')
      }
      (0, 0, 0, 0) => {
        warn!("No delimiters found, defaulting to comma");
        Err(miette::miette!(
          code = "NO_DELIMITER_FOUND",
          help = "The file may not be a delimited format, or it might use an unsupported delimiter. Supported delimiters: comma (,), tab (\\t), pipe (|), semicolon (;)",
          severity = miette::Severity::Warning,
          labels = vec![
            miette::LabeledSpan::at(
              0..first_line.len().saturating_sub(1),
              "analyzed content"
            ),
            miette::LabeledSpan::at(0..0, "no delimiters found")
          ],
          "Could not detect delimiter: no common delimiters found in first line"
        ))
      }
      _ => {
        warn!("Ambiguous delimiter detection, defaulting to comma");
        let analysis = format!(
          "commas:{comma_count}, tabs:{tab_count}, pipes:{pipe_count}, semicolons:{semicolon_count}"
        );
        Err(miette::miette!(
          code = "AMBIGUOUS_DELIMITER",
          help = "Multiple delimiters found with similar counts. Consider manually specifying the delimiter.",
          severity = miette::Severity::Warning,
          labels = vec![
            miette::LabeledSpan::at(
              0..first_line.len().saturating_sub(1),
              "analyzed line"
            ),
            miette::LabeledSpan::at(0..analysis.len(), "delimiter counts")
          ],
          "Ambiguous delimiter detection: {}",
          analysis
        ))?;
        Ok(b',') // This won't be reached due to the ? above
      }
    }
  };

  trace_fn!("assume_delimiter", result())
}
