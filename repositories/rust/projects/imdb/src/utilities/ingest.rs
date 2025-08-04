//! # Generic Data Ingestion Module
//!
//! This module provides reusable ingestion logic for loading source data
//! into DataFusion DataFrames within the bronze data layer.
//! It supports delimiter-separated formats (CSV/TSV/custom) and Parquet.
//!
//! ## Module Overview
//!
//! - `source_to_frame`: Ingests delimiter-separated files with schema inference.
//! - `parquet_to_frame`: Loads Parquet files with schema inference.
//!
//! Both functions:
//! - Accept file paths or dataset configs.
//! - Infer schema lazily.
//! - Handle errors with descriptive context.
//!
//! ## Design Principles
//!
//! - Configuration-Driven: All parsing behavior is controlled via `Config` or
//!   function arguments.
//! - Reusability: Dataset-agnostic utilities for any input.
//! - Performance: Lazy execution, controlled schema sampling.
//! - Observability: Hooks for logging and consistent error messages.

use super::*;

/// Reads a delimiter-separated dataset (CSV/TSV/custom) into a DataFrame.
///
/// This function uses the provided `Config` to apply dataset-agnostic
/// ingestion logic. It builds a regular expression for null value detection,
/// configures CSV parsing options, infers the schema, and returns a
/// DataFusion `DataFrame`.
///
/// # Arguments
///
/// * `config` – Reference to `Config` containing:
///   - `path_as_str()`: Source file path.
///   - `delimiter`: Character for field separation.
///   - `source_ext`: Expected file extension (e.g., "csv", "tsv").
///   - `null_values`: Values considered as null.
///   - `name`: Dataset identifier for logging.
///
/// # Returns
///
/// * `Result<DataFrame>` – On success, a lazy-evaluated `DataFrame` ready
///   for further transformations.
///
/// # Errors
///
/// Returns an error if any of the following occur:
/// - Invalid or inaccessible file path.
/// - File extension mismatch.
/// - Schema inference failure (e.g., inconsistent data types).
/// - Parsing errors encountered by DataFusion.
/// - File permission issues.
///
/// # Performance
///
/// - Limits schema inference to a sample of 1,000 rows (DataFusion default).
/// - Uses lazy evaluation to optimize memory and compute usage.
/// - Assumes the first record contains headers.
///
/// # Example
///
/// ```rust
/// let ratings_cfg = Config::new("Ratings")?;
/// let ratings_df = source_to_frame(&ratings_cfg).await?;
///
/// let titles_cfg = Config::new("Titles")?;
/// let titles_df = source_to_frame(&titles_cfg).await?;
/// ```
///
/// # Integration
///
/// Typically invoked from dataset-specific modules:
///
/// ```rust
/// let df = crate::bronze::tasks::ingest::source_to_frame(&config).await?;
/// ```
pub async fn source_to_frame(
  context: &SessionContext,
  config: &Config,
  name: &str,
) -> Result<DataFrame> {
  let dataset = config
    .datasets
    .all()
    .iter()
    .find(|d| d.name.eq_ignore_ascii_case(name))
    .wrap_err(format!("Unknown dataset name provided: {name}"))?;

  let src_path = dataset.files.raw.as_path();
  let src_path = Path::new("unknown-file-path");
  // let file_analysis = analyze_file(src_path)?;
  let file_analysis = FileAnalysis::new(src_path)?;

  info!("{}", file_analysis);
  bail!("TESTING, TESTING 1, 3");
  debug!("Framing from path {:?}", src_path);

  // Single call to analyze file - gets extension, delimiter, and column names
  let config = &config.ingestion;
  let dataset_name = &dataset.name;

  warn!("Didnt fail");

  if file_analysis.column_names.is_empty() {
    return Err(miette!(
      help =
        "Check if the delimiter is correct and the file has a proper header",
      "No column names found in file header"
    ));
  }

  // Create a schema where ALL columns are nullable strings
  let string_fields: Vec<Field> = file_analysis
    .column_names
    .iter()
    .map(|name| Field::new(name.trim(), DataType::Utf8, true))
    .collect();
  let string_schema = Arc::new(Schema::new(string_fields));

  // Build null regex pattern
  // let null_regex_pattern = Some(format!(
  //   "^({})$",
  //   &config
  //     .null_values
  //     .iter()
  //     .filter(|v| !v.is_empty())
  //     .map(|v| escape(v))
  //     .collect::<Vec<_>>()
  //     .join("|")
  // ));
  let null_regex_pattern = Some(r"\\N".to_string());

  // Read file with detected settings
  let df = context
    .read_csv(
      &file_analysis.path_str,
      CsvReadOptions::new()
        .delimiter(file_analysis.delimiter)
        .file_extension(&file_analysis.extension)
        .has_header(true)
        .null_regex(null_regex_pattern)
        .schema_infer_max_records(1000)
        .schema(&string_schema),
    )
    .await
    .map_err(|e| {
      miette!(
        help = "Common causes:
  • Inconsistent number of fields across rows
  • Invalid characters or encoding issues
  • Memory constraints for very large files
  • Mismatched delimiter configuration",
        "Failed to read delimited data into DataFrame"
      )
      .wrap_err(e)
    })?;

  let actual_schema = df.schema();
  if actual_schema.fields().len() != file_analysis.column_names.len() {
    return Err(miette!(
      "Schema mismatch: expected {} columns, got {}",
      file_analysis.column_names.len(),
      actual_schema.fields().len()
    ));
  }

  Ok(df)
}
