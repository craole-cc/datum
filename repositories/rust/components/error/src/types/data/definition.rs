// -- Error Definition (types/data/definition.rs) -- //
use super::*;

/// Data engineering errors for operations involving DataFrames, Parquet files,
/// data pipelines, transformations, and ETL operations.
///
/// This covers all common data engineering scenarios organized by category:
/// - Schema validation and conflicts
/// - Data type conversion and casting errors
/// - Column operations (missing, duplicate, type mismatches)
/// - DataFrame operations (joins, aggregations, transformations)
/// - Parquet file operations (read, write, schema evolution)
/// - Data pipeline and ETL errors
/// - Data quality and validation issues
/// - Memory and performance errors
/// - Configuration and context errors
#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
  // ================ Schema Errors ================
  #[error("Schema mismatch: expected {expected}, got {actual}")]
  SchemaMismatch { expected: String, actual: String },

  #[error("Schema evolution not supported: {reason}")]
  SchemaEvolutionFailed { reason: String },

  #[error("Invalid schema definition: {details}")]
  InvalidSchema { details: String },

  #[error("Schema conflict between sources: {conflict}")]
  SchemaConflict { conflict: String },

  #[error("Required schema field '{field}' is missing")]
  MissingSchemaField { field: String },

  #[error(
    "Schema field '{field}' has unexpected type: expected {expected}, got {actual}"
  )]
  SchemaFieldTypeMismatch {
    field: String,
    expected: String,
    actual: String,
  },

  // ================ Column Errors ================
  #[error("Column '{column}' not found in DataFrame")]
  ColumnNotFound { column: String },

  #[error("Duplicate column '{column}' detected")]
  DuplicateColumn { column: String },

  #[error(
    "Column '{column}' has wrong data type: expected {expected}, got {actual}"
  )]
  ColumnTypeMismatch {
    column: String,
    expected: String,
    actual: String,
  },

  #[error("Cannot perform operation on column '{column}': {reason}")]
  ColumnOperationFailed { column: String, reason: String },

  #[error("Column '{column}' contains null values but nulls not allowed")]
  UnexpectedNulls { column: String },

  #[error("Column '{column}' is empty but values required")]
  EmptyColumn { column: String },

  #[error("Column name '{column}' is invalid: {reason}")]
  InvalidColumnName { column: String, reason: String },

  #[error("Columns have incompatible shapes: {details}")]
  IncompatibleColumnShapes { details: String },

  #[error(
    "Column '{column}' has invalid values: {invalid_count} out of {total_count}"
  )]
  InvalidColumnValues {
    column: String,
    invalid_count: usize,
    total_count: usize,
  },

  // ================ DataFrame Operation Errors ================
  #[error("DataFrame is empty but operation requires data")]
  EmptyDataFrame,

  #[error(
    "DataFrame shape mismatch: expected {expected_rows}×{expected_cols}, got {actual_rows}×{actual_cols}"
  )]
  DataFrameShapeMismatch {
    expected_rows: usize,
    expected_cols: usize,
    actual_rows: usize,
    actual_cols: usize,
  },

  #[error("Join operation failed: {reason}")]
  JoinFailed { reason: String },

  #[error("Join key '{key}' not found in {side} DataFrame")]
  JoinKeyNotFound { key: String, side: String },

  #[error(
    "Join keys have incompatible types: left '{left_type}', right '{right_type}'"
  )]
  JoinKeyTypeMismatch {
    left_type: String,
    right_type: String,
  },

  #[error("Aggregation failed on column '{column}': {reason}")]
  AggregationFailed { column: String, reason: String },

  #[error("Group by operation failed: {reason}")]
  GroupByFailed { reason: String },

  #[error("Window function '{function}' failed: {reason}")]
  WindowFunctionFailed { function: String, reason: String },

  #[error("Pivot operation failed: {reason}")]
  PivotFailed { reason: String },

  #[error("Sort operation failed on columns [{columns}]: {reason}")]
  SortFailed { columns: String, reason: String },

  #[error("Filter operation failed: {reason}")]
  FilterFailed { reason: String },

  #[error("Row selection failed: {reason}")]
  RowSelectionFailed { reason: String },

  #[error("DataFrame concatenation failed: {reason}")]
  ConcatenationFailed { reason: String },

  // ================ Data Type Conversion Errors ================
  #[error(
    "Cannot cast column '{column}' from {from_type} to {to_type}: {reason}"
  )]
  CastFailed {
    column: String,
    from_type: String,
    to_type: String,
    reason: String,
  },

  #[error(
    "Data type '{data_type}' is not supported for operation '{operation}'"
  )]
  UnsupportedDataType {
    data_type: String,
    operation: String,
  },

  #[error("Type inference failed for column '{column}': {reason}")]
  TypeInferenceFailed { column: String, reason: String },

  #[error("Cannot convert value '{value}' to type {target_type}: {reason}")]
  ValueConversionFailed {
    value: String,
    target_type: String,
    reason: String,
  },

  #[error("Numeric overflow/underflow in column '{column}': {details}")]
  NumericOverflow { column: String, details: String },

  #[error(
    "Invalid date/time format in column '{column}': expected {expected}, got '{actual}'"
  )]
  DateTimeFormat {
    column: String,
    expected: String,
    actual: String,
  },

  #[error("String encoding error in column '{column}': {details}")]
  StringEncoding { column: String, details: String },

  // ================ Parquet File Errors ================
  #[error("Failed to read Parquet file '{path}': {reason}")]
  ParquetReadFailed { path: PathBuf, reason: String },

  #[error("Failed to write Parquet file '{path}': {reason}")]
  ParquetWriteFailed { path: PathBuf, reason: String },

  #[error("Parquet file '{path}' has corrupted metadata")]
  ParquetCorrupted { path: PathBuf },

  #[error("Parquet schema incompatible with DataFrame schema: {details}")]
  ParquetSchemaIncompatible { details: String },

  #[error("Parquet compression '{compression}' not supported")]
  ParquetCompressionNotSupported { compression: String },

  #[error("Parquet row group {row_group} is invalid: {reason}")]
  ParquetRowGroupInvalid { row_group: usize, reason: String },

  #[error("Parquet statistics are missing or invalid for column '{column}'")]
  ParquetStatisticsInvalid { column: String },

  #[error("Parquet file version {version} is not supported")]
  ParquetVersionUnsupported { version: String },

  // ================ Data Pipeline / ETL Errors ================
  #[error("Pipeline step '{step}' failed: {reason}")]
  PipelineStepFailed { step: String, reason: String },

  #[error("Pipeline validation failed: {details}")]
  PipelineValidationFailed { details: String },

  #[error("Data source '{resource}' is not available: {reason}")]
  DataSourceUnavailable { resource: String, reason: String },

  #[error("Data sink '{sink}' failed to accept data: {reason}")]
  DataSinkFailed { sink: String, reason: String },

  #[error("ETL transformation '{transformation}' failed: {reason}")]
  TransformationFailed {
    transformation: String,
    reason: String,
  },

  #[error("Data lineage tracking failed: {reason}")]
  LineageTrackingFailed { reason: String },

  #[error("Batch processing failed at batch {batch_id}: {reason}")]
  BatchProcessingFailed { batch_id: String, reason: String },

  #[error("Stream processing failed: {reason}")]
  StreamProcessingFailed { reason: String },

  #[error("Data dependency '{dependency}' not satisfied: {reason}")]
  DependencyNotSatisfied { dependency: String, reason: String },

  // ================ Data Quality Errors ================
  #[error("Data quality check '{check}' failed: {details}")]
  DataQualityFailed { check: String, details: String },

  #[error("Constraint violation in column '{column}': {constraint}")]
  ConstraintViolation { column: String, constraint: String },

  #[error(
    "Data completeness check failed: {missing_percentage:.2}% missing values in '{column}' (threshold: {threshold:.2}%)"
  )]
  CompletenessCheckFailed {
    column: String,
    missing_percentage: f64,
    threshold: f64,
  },

  #[error(
    "Data uniqueness violation: {duplicate_count} duplicates found in '{column}' (expected unique)"
  )]
  UniquenessViolation {
    column: String,
    duplicate_count: usize,
  },

  #[error(
    "Data range validation failed for column '{column}': value {value} outside range [{min}, {max}]"
  )]
  RangeValidationFailed {
    column: String,
    value: String,
    min: String,
    max: String,
  },

  #[error(
    "Data pattern validation failed for column '{column}': '{value}' does not match pattern '{pattern}'"
  )]
  PatternValidationFailed {
    column: String,
    value: String,
    pattern: String,
  },

  #[error("Referential integrity violation: {details}")]
  ReferentialIntegrityViolation { details: String },

  #[error(
    "Data freshness check failed: data is {age_hours} hours old (threshold: {threshold_hours} hours)"
  )]
  DataFreshnessCheckFailed {
    age_hours: u64,
    threshold_hours: u64,
  },

  // ================ LazyFrame Execution Errors ================
  #[error("LazyFrame execution failed: {reason}")]
  LazyFrameExecutionFailed { reason: String },

  #[error("Query optimization failed: {reason}")]
  QueryOptimizationFailed { reason: String },

  #[error("Lazy operation '{operation}' is not supported: {reason}")]
  LazyOperationNotSupported { operation: String, reason: String },

  #[error("Predicate pushdown failed: {reason}")]
  PredicatePushdownFailed { reason: String },

  #[error("Projection pushdown failed: {reason}")]
  ProjectionPushdownFailed { reason: String },

  #[error("Query plan generation failed: {reason}")]
  QueryPlanFailed { reason: String },

  // ================ Memory and Performance Errors ================
  #[error("Out of memory during operation '{operation}': {details}")]
  OutOfMemory { operation: String, details: String },

  #[error("Operation timeout after {timeout_seconds}s: {operation}")]
  OperationTimeout {
    operation: String,
    timeout_seconds: u64,
  },

  #[error(
    "Resource limit exceeded: {limit_type} limit of {limit_value} exceeded"
  )]
  ResourceLimitExceeded {
    limit_type: String,
    limit_value: String,
  },

  #[error("Threading error in parallel operation: {details}")]
  ParallelThread { details: String },

  #[error(
    "Performance degradation detected: {operation} took {actual_ms}ms (expected < {threshold_ms}ms)"
  )]
  PerformanceDegradation {
    operation: String,
    actual_ms: u64,
    threshold_ms: u64,
  },

  // ================ Configuration and Context Errors ================
  #[error("Invalid configuration for '{component}': {reason}")]
  InvalidConfiguration { component: String, reason: String },

  #[error("Missing required configuration: '{key}'")]
  MissingConfiguration { key: String },

  #[error("Data source connection failed: {resource}")]
  ConnectionFailed { resource: String },

  #[error("Authentication failed for data source '{resource}': {reason}")]
  AuthenticationFailed { resource: String, reason: String },

  #[error("Permission denied for operation '{operation}' on '{resource}'")]
  PermissionDenied { operation: String, resource: String },

  #[error("Environment error: {details}")]
  Environment { details: String },

  // ================ Multiple Errors ================
  #[error("Multiple data errors occurred ({count} errors):\n{errors:#?}")]
  Multiple { count: usize, errors: Vec<Error> },

  // ================ Generic/Context Errors ================
  #[error("Data operation failed: {message}")]
  Context { message: String },

  #[error("Polars error: {details}")]
  Polars { details: String },

  #[error("Arrow error: {details}")]
  Arrow { details: String },

  #[error("External library error: {library} - {details}")]
  ExtLib { library: String, details: String },
}

/// Error category for better error organization and handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
  Schema,
  Column,
  DataFrame,
  DataType,
  Parquet,
  Pipeline,
  DataQuality,
  LazyFrame,
  Performance,
  Configuration,
  System,
}

/// A builder for collecting multiple data validation errors
#[derive(Debug, Default)]
pub struct Builder {
  errors: Vec<Error>,
  continue_on_error: bool,
}

/// Result type alias for data operations
pub type Result<T> = result::Result<T, Error>;
