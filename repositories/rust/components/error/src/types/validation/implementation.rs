// -- Implementation (types/data/implementation.rs) -- //

use super::*;

impl Error {
  /// Get the severity level of this error
  pub fn severity(&self) -> Severity {
    match self {
      // Critical errors - system failures
      Self::OutOfMemory { .. } | Self::ParallelThread { .. } => {
        Severity::Critical
      }

      // High severity - operation failures
      Self::ParquetCorrupted { .. }
      | Self::ConnectionFailed { .. }
      | Self::AuthenticationFailed { .. }
      | Self::PipelineStepFailed { .. }
      | Self::LazyFrameExecutionFailed { .. } => Severity::High,

      // Medium severity - data issues that stop processing
      Self::ColumnNotFound { .. }
      | Self::JoinFailed { .. }
      | Self::CastFailed { .. }
      | Self::DataQualityFailed { .. }
      | Self::ConstraintViolation { .. } => Severity::Medium,

      // Low severity - warnings and minor issues
      Self::PerformanceDegradation { .. }
      | Self::DataFreshnessCheckFailed { .. } => Severity::Low,

      // Multiple errors inherit highest severity
      Self::Multiple { errors, .. } => errors
        .iter()
        .map(|e| e.severity())
        .max()
        .unwrap_or(Severity::Medium),

      // Default to medium for other errors
      _ => Severity::Medium,
    }
  }

  /// Get the category of this error
  pub fn category(&self) -> Category {
    match self {
      Self::SchemaMismatch { .. }
      | Self::SchemaEvolutionFailed { .. }
      | Self::InvalidSchema { .. }
      | Self::SchemaConflict { .. }
      | Self::MissingSchemaField { .. }
      | Self::SchemaFieldTypeMismatch { .. }
      | Self::ParquetSchemaIncompatible { .. } => Category::Schema,

      Self::ColumnNotFound { .. }
      | Self::DuplicateColumn { .. }
      | Self::ColumnTypeMismatch { .. }
      | Self::ColumnOperationFailed { .. }
      | Self::UnexpectedNulls { .. }
      | Self::EmptyColumn { .. }
      | Self::InvalidColumnName { .. }
      | Self::IncompatibleColumnShapes { .. }
      | Self::InvalidColumnValues { .. } => Category::Column,

      Self::EmptyDataFrame
      | Self::DataFrameShapeMismatch { .. }
      | Self::JoinFailed { .. }
      | Self::JoinKeyNotFound { .. }
      | Self::JoinKeyTypeMismatch { .. }
      | Self::AggregationFailed { .. }
      | Self::GroupByFailed { .. }
      | Self::WindowFunctionFailed { .. }
      | Self::PivotFailed { .. }
      | Self::SortFailed { .. }
      | Self::FilterFailed { .. }
      | Self::RowSelectionFailed { .. }
      | Self::ConcatenationFailed { .. } => Category::DataFrame,

      Self::CastFailed { .. }
      | Self::UnsupportedDataType { .. }
      | Self::TypeInferenceFailed { .. }
      | Self::ValueConversionFailed { .. }
      | Self::NumericOverflow { .. }
      | Self::DateTimeFormat { .. }
      | Self::StringEncoding { .. } => Category::DataType,

      Self::ParquetReadFailed { .. }
      | Self::ParquetWriteFailed { .. }
      | Self::ParquetCorrupted { .. }
      | Self::ParquetCompressionNotSupported { .. }
      | Self::ParquetRowGroupInvalid { .. }
      | Self::ParquetStatisticsInvalid { .. }
      | Self::ParquetVersionUnsupported { .. } => Category::Parquet,

      Self::PipelineStepFailed { .. }
      | Self::PipelineValidationFailed { .. }
      | Self::DataSourceUnavailable { .. }
      | Self::DataSinkFailed { .. }
      | Self::TransformationFailed { .. }
      | Self::LineageTrackingFailed { .. }
      | Self::BatchProcessingFailed { .. }
      | Self::StreamProcessingFailed { .. }
      | Self::DependencyNotSatisfied { .. } => Category::Pipeline,

      Self::DataQualityFailed { .. }
      | Self::ConstraintViolation { .. }
      | Self::CompletenessCheckFailed { .. }
      | Self::UniquenessViolation { .. }
      | Self::RangeValidationFailed { .. }
      | Self::PatternValidationFailed { .. }
      | Self::ReferentialIntegrityViolation { .. }
      | Self::DataFreshnessCheckFailed { .. } => Category::DataQuality,

      Self::LazyFrameExecutionFailed { .. }
      | Self::QueryOptimizationFailed { .. }
      | Self::LazyOperationNotSupported { .. }
      | Self::PredicatePushdownFailed { .. }
      | Self::ProjectionPushdownFailed { .. }
      | Self::QueryPlanFailed { .. } => Category::LazyFrame,

      Self::OutOfMemory { .. }
      | Self::OperationTimeout { .. }
      | Self::ResourceLimitExceeded { .. }
      | Self::ParallelThread { .. }
      | Self::PerformanceDegradation { .. } => Category::Performance,

      Self::InvalidConfiguration { .. }
      | Self::MissingConfiguration { .. }
      | Self::ConnectionFailed { .. }
      | Self::AuthenticationFailed { .. }
      | Self::PermissionDenied { .. }
      | Self::Environment { .. } => Category::Configuration,

      _ => Category::System,
    }
  }

  /// Check if this error is recoverable (can be retried)
  pub fn is_recoverable(&self) -> bool {
    match self {
      Self::ConnectionFailed { .. }
      | Self::DataSourceUnavailable { .. }
      | Self::OperationTimeout { .. }
      | Self::OutOfMemory { .. }
      | Self::ParallelThread { .. }
      | Self::DataSinkFailed { .. }
      | Self::BatchProcessingFailed { .. } => true,

      Self::Multiple { errors, .. } => {
        errors.iter().any(|e| e.is_recoverable())
      }

      _ => false,
    }
  }

  /// Check if this error is a schema-related issue
  pub fn is_schema_error(&self) -> bool {
    matches!(self.category(), Category::Schema)
  }

  /// Check if this error is related to data quality
  pub fn is_data_quality_error(&self) -> bool {
    matches!(self.category(), Category::DataQuality)
  }

  /// Check if this error is related to column operations
  pub fn is_column_error(&self) -> bool {
    matches!(self.category(), Category::Column)
  }

  /// Check if this error should be logged at warning level
  pub fn is_warning(&self) -> bool {
    matches!(self.severity(), Severity::Low)
  }

  /// Get the column name if this error is column-specific
  pub fn column_name(&self) -> Option<&str> {
    match self {
      Self::ColumnNotFound { column }
      | Self::DuplicateColumn { column }
      | Self::ColumnTypeMismatch { column, .. }
      | Self::ColumnOperationFailed { column, .. }
      | Self::UnexpectedNulls { column }
      | Self::EmptyColumn { column }
      | Self::InvalidColumnName { column, .. }
      | Self::InvalidColumnValues { column, .. }
      | Self::CastFailed { column, .. }
      | Self::NumericOverflow { column, .. }
      | Self::DateTimeFormat { column, .. }
      | Self::StringEncoding { column, .. }
      | Self::TypeInferenceFailed { column, .. }
      | Self::AggregationFailed { column, .. }
      | Self::ParquetStatisticsInvalid { column }
      | Self::ConstraintViolation { column, .. }
      | Self::CompletenessCheckFailed { column, .. }
      | Self::UniquenessViolation { column, .. }
      | Self::RangeValidationFailed { column, .. }
      | Self::PatternValidationFailed { column, .. } => Some(column),
      _ => None,
    }
  }

  /// Get related error context information
  pub fn context_info(&self) -> HashMap<String, String> {
    let mut context = HashMap::new();

    context.insert("category".to_string(), format!("{:?}", self.category()));
    context.insert("severity".to_string(), format!("{:?}", self.severity()));
    context
      .insert("recoverable".to_string(), self.is_recoverable().to_string());

    if let Some(column) = self.column_name() {
      context.insert("column".to_string(), column.to_string());
    }

    match self {
      Self::DataFrameShapeMismatch {
        expected_rows,
        expected_cols,
        actual_rows,
        actual_cols,
      } => {
        context.insert(
          "expected_shape".to_string(),
          format!("{expected_rows}×{expected_cols}"),
        );
        context.insert(
          "actual_shape".to_string(),
          format!("{actual_rows}×{actual_cols}"),
        );
      }
      Self::Multiple { count, .. } => {
        context.insert("error_count".to_string(), count.to_string());
      }
      Self::ParquetReadFailed { path, .. }
      | Self::ParquetWriteFailed { path, .. } => {
        context.insert("file_path".to_string(), path.display().to_string());
      }
      _ => {}
    }

    context
  }

  /// Combine multiple errors into a single error
  pub fn combine(errors: Vec<Error>) -> Self {
    if errors.is_empty() {
      return Self::context("No errors to combine");
    }

    if errors.len() == 1 {
      return errors.into_iter().next().unwrap();
    }

    let count = errors.len();
    Self::Multiple { count, errors }
  }

  // ================ Constructor Methods ================

  // -- Schema errors
  pub fn schema_mismatch<S: Into<String>>(expected: S, actual: S) -> Self {
    Self::SchemaMismatch {
      expected: expected.into(),
      actual: actual.into(),
    }
  }

  pub fn schema_evolution_failed<S: Into<String>>(reason: S) -> Self {
    Self::SchemaEvolutionFailed {
      reason: reason.into(),
    }
  }

  // -- Column errors
  pub fn column_not_found<S: Into<String>>(column: S) -> Self {
    Self::ColumnNotFound {
      column: column.into(),
    }
  }

  pub fn column_type_mismatch<S: Into<String>>(
    column: S,
    expected: S,
    actual: S,
  ) -> Self {
    Self::ColumnTypeMismatch {
      column: column.into(),
      expected: expected.into(),
      actual: actual.into(),
    }
  }

  pub fn duplicate_column<S: Into<String>>(column: S) -> Self {
    Self::DuplicateColumn {
      column: column.into(),
    }
  }

  // -- DataFrame errors
  pub fn join_failed<S: Into<String>>(reason: S) -> Self {
    Self::JoinFailed {
      reason: reason.into(),
    }
  }

  pub fn aggregation_failed<S: Into<String>>(column: S, reason: S) -> Self {
    Self::AggregationFailed {
      column: column.into(),
      reason: reason.into(),
    }
  }

  pub fn window_function_failed<S: Into<String>>(
    function: S,
    reason: S,
  ) -> Self {
    Self::WindowFunctionFailed {
      function: function.into(),
      reason: reason.into(),
    }
  }

  // -- Type errors
  pub fn cast_failed<S: Into<String>>(
    column: S,
    from_type: S,
    to_type: S,
    reason: S,
  ) -> Self {
    Self::CastFailed {
      column: column.into(),
      from_type: from_type.into(),
      to_type: to_type.into(),
      reason: reason.into(),
    }
  }

  // -- Parquet errors
  pub fn parquet_read_failed<P: Into<PathBuf>, S: Into<String>>(
    path: P,
    reason: S,
  ) -> Self {
    Self::ParquetReadFailed {
      path: path.into(),
      reason: reason.into(),
    }
  }

  pub fn parquet_write_failed<P: Into<PathBuf>, S: Into<String>>(
    path: P,
    reason: S,
  ) -> Self {
    Self::ParquetWriteFailed {
      path: path.into(),
      reason: reason.into(),
    }
  }

  // -- Quality errors
  pub fn data_quality_failed<S: Into<String>>(check: S, details: S) -> Self {
    Self::DataQualityFailed {
      check: check.into(),
      details: details.into(),
    }
  }

  pub fn constraint_violation<S: Into<String>>(
    column: S,
    constraint: S,
  ) -> Self {
    Self::ConstraintViolation {
      column: column.into(),
      constraint: constraint.into(),
    }
  }

  pub fn completeness_check_failed<S: Into<String>>(
    column: S,
    missing_percentage: f64,
    threshold: f64,
  ) -> Self {
    Self::CompletenessCheckFailed {
      column: column.into(),
      missing_percentage,
      threshold,
    }
  }

  // -- Performance errors
  pub fn operation_timeout<S: Into<String>>(
    operation: S,
    timeout_seconds: u64,
  ) -> Self {
    Self::OperationTimeout {
      operation: operation.into(),
      timeout_seconds,
    }
  }

  pub fn out_of_memory<S: Into<String>>(operation: S, details: S) -> Self {
    Self::OutOfMemory {
      operation: operation.into(),
      details: details.into(),
    }
  }

  // -- Generic constructors
  pub fn context<S: Into<String>>(message: S) -> Self {
    Self::Context {
      message: message.into(),
    }
  }

  pub fn polars_error<S: Into<String>>(details: S) -> Self {
    Self::Polars {
      details: details.into(),
    }
  }

  pub fn arrow_error<S: Into<String>>(details: S) -> Self {
    Self::Arrow {
      details: details.into(),
    }
  }

  pub fn external_library_error<S: Into<String>>(
    library: S,
    details: S,
  ) -> Self {
    Self::ExtLib {
      library: library.into(),
      details: details.into(),
    }
  }
}
