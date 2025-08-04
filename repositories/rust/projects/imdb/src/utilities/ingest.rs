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
/// * `TheResult<DataFrame>` – On success, a lazy-evaluated `DataFrame` ready
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
pub async fn source_to_frame_miette(config: &Config) -> TheResult<DataFrame> {
  // Read the first line to get column names
  let file = File::open(config.path_as_str()?).map_err(|e| {
    miette!(
      help = "Check file path, permissions, and that the file exists",
      "Cannot open CSV file: {}",
      config.path_as_str().unwrap_or("unknown")
    )
    .wrap_err(e)
  })?;

  let mut reader = BufReader::new(file);
  let mut header_line = String::new();
  reader.read_line(&mut header_line).map_err(|e| {
    miette!(
      help = "File might be empty, corrupted, or have encoding issues",
      "Cannot read header line from CSV file"
    )
    .wrap_err(e)
  })?;

  // Parse column names
  let column_names: Vec<&str> =
    header_line.trim().split(config.delimiter as char).collect();

  if column_names.is_empty() {
    return Err(miette!(
      help =
        "Check if the delimiter is correct and the file has a proper header",
      "No column names found in CSV header"
    ));
  }

  // Create a schema where ALL columns are nullable strings
  let string_fields: Vec<Field> = column_names
    .iter()
    .map(|name| Field::new(name.trim(), DataType::Utf8, true))
    .collect();

  let string_schema = Arc::new(Schema::new(string_fields));

  // Build null regex pattern
  let null_regex_pattern = Some(format!(
    "^({})$",
    config
      .null_values
      .iter()
      .filter(|v| !v.is_empty())
      .map(|v| regex::escape(v))
      .collect::<Vec<_>>()
      .join("|")
  ));

  // Create CSV format with the string schema and null handling
  let csv_format = CsvFormat::default()
    .with_delimiter(config.delimiter)
    .with_has_header(true)
    .with_null_regex(null_regex_pattern);

  // Initialize DataFusion context
  let ctx = SessionContext::new();
  let session_state = ctx.state();

  // Prepare listing options with explicit schema
  let listing_options = ListingOptions::new(Arc::new(csv_format))
    .with_file_extension(&config.source_ext);

  // Parse the path into a DataFusion ListingTableUrl
  let table_url =
    ListingTableUrl::parse(config.path_as_str()?).map_err(|e| {
      miette!(
        help = "Check that the file path is valid and accessible",
        "Invalid table URL for path: {}",
        config.path_as_str().unwrap_or("unknown")
      )
      .wrap_err(e)
    })?;

  // Build the listing table configuration with our string schema
  let table_config = ListingTableConfig::new(table_url)
    .with_listing_options(listing_options)
    .with_schema(string_schema);

  // Create the listing table provider
  let table_provider =
    Arc::new(ListingTable::try_new(table_config).map_err(|e| {
      miette!(
        help = "This usually indicates schema or file format issues",
        "Failed to create DataFrame table provider"
      )
      .wrap_err(e)
    })?);

  // Read the table into a DataFrame
  let df = ctx.read_table(table_provider).map_err(|e| {
    miette!(
      help = "Common causes:
• Inconsistent number of fields across CSV rows
• Invalid characters or encoding issues
• Memory constraints for very large files
• Mismatched delimiter configuration",
      "Failed to read CSV data into DataFrame"
    )
    .wrap_err(e)
  })?;

  debug!(
    "Loaded schema for '{}' with all string columns",
    config.name
  );
  Ok(df)
}
pub async fn source_to_frame(config: &Config) -> TheResult<DataFrame> {
  // Initialize a new DataFusion session.
  let ctx = SessionContext::new();

  // Build a regex pattern from the configured null values.
  let null_regex_pattern = Some(format!(
    "^({})$",
    config
      .null_values
      .iter()
      .filter(|v| !v.is_empty())
      .map(|v| regex::escape(v))
      .collect::<Vec<_>>()
      .join("|")
  ));

  // Read the first line to get column names
  let file = File::open(config.path_as_str()?)
    .into_diagnostic()
    .wrap_err("Failed to open CSV file")?;

  let mut reader = BufReader::new(file);
  let mut header_line = String::new();
  reader
    .read_line(&mut header_line)
    .into_diagnostic()
    .wrap_err("Failed to read header line")?;

  // Parse column names
  let column_names: Vec<&str> =
    header_line.trim().split(config.delimiter as char).collect();

  // Create a schema where ALL columns are nullable strings
  let string_fields: Vec<Field> = column_names
    .iter()
    .map(|name| Field::new(name.trim(), DataType::Utf8, true))
    .collect();

  let string_schema = Arc::new(Schema::new(string_fields));

  // Create CSV format with the string schema and null handling
  let csv_format = CsvFormat::default()
    .with_delimiter(config.delimiter)
    .with_has_header(true)
    .with_null_regex(null_regex_pattern);

  // Prepare listing options with explicit schema
  let listing_options = ListingOptions::new(Arc::new(csv_format))
    .with_file_extension(&config.source_ext);

  // Parse the path into a DataFusion ListingTableUrl
  let table_url = ListingTableUrl::parse(config.path_as_str()?)
    .into_diagnostic()
    .wrap_err("Failed to parse table URL")?;

  // Build the listing table configuration with our string schema
  let table_config = ListingTableConfig::new(table_url)
    .with_listing_options(listing_options)
    .with_schema(string_schema);

  // Create the listing table provider
  let table_provider = Arc::new(
    ListingTable::try_new(table_config)
      .into_diagnostic()
      .wrap_err("Failed to create listing table")?,
  );

  // Read the table into a DataFrame
  let df = ctx
    .read_table(table_provider)
    .into_diagnostic()
    .wrap_err(format!("Failed to read the source file: {}", config.name))?;

  debug!(
    "Loaded schema for '{}' with all string columns",
    config.name
  );
  Ok(df)
}

// Helper function to make all schema fields nullable
fn make_all_fields_nullable(schema: &Schema) -> Schema {
  let nullable_fields: Vec<Field> = schema
    .fields()
    .iter()
    .map(|field| {
      Field::new(
        field.name(),
        field.data_type().clone(),
        true, // Make nullable
      )
      .with_metadata(field.metadata().clone())
    })
    .collect();

  Schema::new_with_metadata(nullable_fields, schema.metadata().clone())
}

pub async fn source_to_frame_1(config: &Config) -> TheResult<DataFrame> {
  // Initialize a new DataFusion session.
  let ctx = SessionContext::new();
  let session_state = ctx.state();

  // Build a regex pattern from the configured null values.
  // Example: if null_values = ["\\N", "NULL"], pattern = "^(\\N|NULL)$"
  let null_regex_pattern = Some(format!(
    "^({})$",
    config
      .null_values
      .iter()
      .filter(|v| !v.is_empty()) // avoid empty strings
      .map(|v| regex::escape(v))
      .collect::<Vec<_>>()
      .join("|")
  ));

  // Configure CSV parsing options with custom delimiter and null regex.
  let csv_format = CsvFormat::default()
    .with_delimiter(config.delimiter)
    .with_has_header(true)
    .with_null_regex(null_regex_pattern)
    .with_schema_infer_max_rec(100_000);
  // warn!("{:#?}", csv_format);

  // Prepare listing options, enforcing the expected file extension.
  let listing_options = ListingOptions::new(Arc::new(csv_format))
    .with_file_extension(&config.source_ext);

  // Parse the path into a DataFusion ListingTableUrl.
  let table_url = ListingTableUrl::parse(config.path_as_str()?)
    .into_diagnostic()
    .wrap_err("Failed to parse table URL")?;

  // Infer schema by scanning the source files lazily.
  let inferred_schema = listing_options
    .infer_schema(&session_state, &table_url)
    .await
    .into_diagnostic()
    .wrap_err("Failed to infer schema")?;

  // Update the schema to make all fields nullable
  let patched_schema = Arc::new(make_all_fields_nullable(&inferred_schema));

  // Build the listing table configuration with the patched schema.
  let table_config = ListingTableConfig::new(table_url)
    .with_listing_options(listing_options)
    // .with_schema(inferred_schema);
    .with_schema(patched_schema);

  // Create the listing table provider.
  let table_provider = Arc::new(
    ListingTable::try_new(table_config)
      .into_diagnostic()
      .wrap_err("Failed to create listing table")?,
  );

  // Read the table into a DataFrame.
  let df = ctx
    .read_table(table_provider)
    .into_diagnostic()
    .wrap_err(format!("Failed to read the source file: {}", config.name))?;

  debug!("Loaded schema for '{}'", config.name);
  Ok(df)
}

/// Reads Parquet files into a DataFrame with schema inference.
///
/// This function leverages DataFusion's `ListingTable` API to
/// discover and read Parquet files from the specified path.
/// It infers the Parquet schema and produces a lazy `DataFrame`.
///
/// # Arguments
///
/// * `file_path` – Path or URI to the Parquet file or directory.
///
/// # Returns
///
/// * `TheResult<DataFrame>` – On success, a DataFrame for downstream transforms.
///
/// # Errors
///
/// Returns an error if:
/// - The file path cannot be parsed.
/// - Schema inference fails.
/// - DataFusion encounters read or parse errors.
///
/// # Performance
///
/// - Specifies a single target partition by default.
/// - Lazy evaluation defers actual I/O until an action is executed.
///
/// # Example
///
/// ```rust
/// let df = parquet_to_frame("/data/bronze/users.parquet").await?;
/// ```
pub async fn parquet_to_frame(file_path: &str) -> TheResult<DataFrame> {
  // Initialize DataFusion session.
  let ctx = SessionContext::new();
  let session_state = ctx.state();

  // Configure Parquet format.
  let parquet_format = ParquetFormat::new();

  // Listing options enforcing ".parquet" extension and target partitions.
  let listing_options = ListingOptions::new(Arc::new(parquet_format))
    .with_file_extension("parquet")
    .with_target_partitions(1);

  // Parse the file path into a ListingTableUrl.
  let table_url = ListingTableUrl::parse(file_path)
    .into_diagnostic()
    .wrap_err("Failed to parse parquet file path")?;

  // Infer schema from the Parquet file(s) lazily.
  let resolved_schema = listing_options
    .infer_schema(&session_state, &table_url)
    .await
    .into_diagnostic()
    .wrap_err("Failed to infer parquet schema")?;

  // Assemble the table configuration with inferred schema.
  let table_config = ListingTableConfig::new(table_url)
    .with_listing_options(listing_options)
    .with_schema(resolved_schema);

  // Create the listing table as a provider.
  let table_provider = Arc::new(
    ListingTable::try_new(table_config)
      .into_diagnostic()
      .wrap_err("Failed to create parquet listing table")?,
  );

  // Read into a DataFrame.
  let df = ctx
    .read_table(table_provider)
    .into_diagnostic()
    .wrap_err("Failed to read parquet file")?;

  Ok(df)
}
