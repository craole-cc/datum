//! # Generic Parquet Export Task
//!
//! This module provides reusable Parquet export functionality for the bronze layer.
//! It works with any dataset configuration and DataFrame, handling the common patterns
//! for converting DataFrames to optimized Parquet files with compression.
//!
//! ## Design Philosophy
//!
//! This is a pure utility module that:
//! - Accepts any `Config` object and DataFrame
//! - Applies dataset-agnostic export logic
//! - Handles directory creation and file management
//! - Provides idempotent operations (safe to re-run)
//! - Returns the path to the created Parquet file
//!
//! ## Features
//!
//! - **Configurable Compression**: SNAPPY, GZIP, LZ4, ZSTD support
//! - **Automatic Directory Creation**: Ensures output paths exist
//! - **Idempotent Operations**: Skips export if file already exists
//! - **Comprehensive Error Handling**: Detailed context for failures
//! - **Debug Logging**: Export verification and path information
//!
//! ## Configuration-Driven Behavior
//!
//! All export behavior is controlled through the `Config` object:
//! - `config.import_dir`: Target directory for Parquet files
//! - `config.import_path()`: Full path to output file
//! - `config.parquet_compression`: Compression algorithm
//! - `config.name`: Dataset name for logging

use super::*;

/// Generic function to export DataFrames to Parquet format
///
/// This function provides a reusable export pattern that works with any dataset
/// configuration and DataFrame. It handles the complete export workflow from
/// directory creation through final file verification.
///
/// # Arguments
///
/// * `config` - Dataset configuration containing export paths and compression settings
/// * `frame` - The DataFrame to export to Parquet format
///
/// # Returns
///
/// * `Ok(PathBuf)` - Path to the successfully created Parquet file
/// * `Err(TheError)` - Export operation failed
///
/// # Errors
///
/// This function will return an error if:
/// - Target directory cannot be created due to permissions
/// - Insufficient disk space for the Parquet file
/// - DataFrame contains unsupported data types for Parquet
/// - File system errors during write operations
/// - Parquet serialization fails
///
/// # Idempotency Guarantee
///
/// This function is designed to be idempotent - calling it multiple times with
/// the same configuration will skip the export if the target file already exists.
/// This enables safe re-execution of bronze layer workflows without data duplication.
///
/// # Configuration Dependencies
///
/// This function relies on the following config properties:
/// - `import_dir`: Base directory for Parquet output files
/// - `import_path()`: Method returning the full output file path
/// - `parquet_compression`: Compression algorithm (SNAPPY, GZIP, etc.)
/// - `name`: Dataset identifier for logging and error messages
/// - `ensure_import_dir()`: Method to create output directory
/// - `ensure_import_exists()`: Method to check if file already exists
///
/// # Compression Options
///
/// The compression algorithm is specified via `config.parquet_compression`:
/// - **SNAPPY**: Fast compression/decompression, good balance (recommended)
/// - **GZIP**: Higher compression ratio, slower processing
/// - **LZ4**: Very fast compression, moderate file size reduction
/// - **ZSTD**: Modern algorithm with excellent compression and speed
///
/// # Performance Considerations
///
/// - **CPU Usage**: Compression is CPU-intensive, especially for large DataFrames
/// - **Memory Usage**: Scales with DataFrame size and row group configuration
/// - **I/O Performance**: Benefits from SSD storage for large datasets
/// - **Network Storage**: Higher compression ratios reduce transfer time
///
/// # Examples
///
/// ```rust
/// use crate::config::Config;
/// use crate::bronze::tasks::export::export_parquet;
/// use datafusion::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Works with any dataset configuration and DataFrame
///     let config = Config::new("Ratings")?;
///     let ctx = SessionContext::new();
///     let df = ctx.sql("SELECT * FROM source_table").await?;
///
///     let output_path = export_parquet(&config, df).await?;
///     println!("Exported to: {:?}", output_path);
///
///     // Safe to call again - will skip if file exists
///     let same_path = export_parquet(&config, df).await?;
///     assert_eq!(output_path, same_path);
///     Ok(())
/// }
/// ```
///
/// # Integration with Bronze Workflow
///
/// This function is designed to be called from dataset-specific modules:
///
/// ```rust
/// // In data/ratings.rs, data/titles.rs, etc.
/// let path = crate::bronze::tasks::export::export_parquet(&config, dataframe).await?;
/// ```
pub async fn frame_to_parquet(
  config: &Config,
  frame: DataFrame,
) -> TheResult<PathBuf> {
  let home = &config.import_dir;
  let name = &config.name;
  let path = config.import_path();
  let path_osstr = path.as_os_str();
  let path_str = &path.to_string_lossy();
  let compression = config.parquet_compression.clone();

  // Configure DataFrame writing options
  let frame_opts = DataFrameWriteOptions::new();
  let writer_opts = Some(TableParquetOptions {
    global: ParquetOptions {
      compression,
      ..Default::default()
    },
    ..Default::default()
  });

  // Create the target directory if it doesn't exist
  config.ensure_import_dir()?;

  // Return early if the import is already present (idempotent behavior)
  if config.ensure_import_exists().is_ok() {
    debug!(
      "Skipping parquet export for '{name}' as it already exists at {path_osstr:?}"
    );
    return Ok(path);
  }

  // Attempt to export the dataframe as a Parquet file
  frame
    .write_parquet(path_str, frame_opts, writer_opts)
    .await
    .into_diagnostic()
    .wrap_err(format!("Failed to write parquet file: {path_str:?}"))?;
  // .context("Failed to write Parquet file")?;

  debug!("Successfully exported parquet file for '{name}' to {path_osstr:?}");
  Ok(path)
}
