// -- Data Error Prelude (types/data/prelude.rs) --

use super::*;
use paste::paste;

pub use definition::{
  Category as DataErrorCategory, Error as DataError, Result as DataResult,
};

/// Generates both the local `Error::$Variant` ctor and
/// a `crate::Error::data_$variant(...)` wrapper.
macro_rules! define_data_constructors {
    (
        $(
            $Variant:ident { $($field:ident : $ftype:ty),* $(,)? }
        ),* $(,)?
    ) => {
        paste! {
            // 1) Local ctors on types::data::Error
            impl Error {
                $(
                    #[allow(clippy::too_many_arguments)]
                    pub fn [<$Variant:snake>](
                        $(
                            $field: define_data_constructors!(@arg_ty $ftype)
                        ),*
                    ) -> Self {
                        Error::$Variant {
                            $(
                                $field: define_data_constructors!(@field_init $field, $ftype)
                            ),*
                        }
                    }
                )*
            }

            // 2) Root‐level wrappers on crate::Error
            impl crate::Error {
                $(
                    #[allow(clippy::too_many_arguments)]
                    pub fn [<data_ $Variant:snake>](
                        $(
                            $field: define_data_constructors!(@arg_ty $ftype)
                        ),*
                    ) -> Self {
                        crate::Error::Data(
                            Error::[<$Variant:snake>](
                                $(
                                    define_data_constructors!(@unwrap_arg $field, $ftype)
                                ),*
                            )
                        )
                    }
                )*
            }
        }
    };

    // Accept Into<String> for String fields
    (@arg_ty String) => (impl Into<String>);
    (@arg_ty $other:ty) => ($other);

    (@field_init $field:ident, String) => ($field.into());
    (@field_init $field:ident, $other:ty) => ($field);

    (@unwrap_arg $field:ident, String) => ($field.into());
    (@unwrap_arg $field:ident, $other:ty) => ($field);
}

// List every variant in types/data/definition.rs, with its fields:
define_data_constructors! {
    SchemaMismatch                { expected: String, actual: String },
    SchemaEvolutionFailed         { reason:   String               },
    InvalidSchema                 { details:  String               },
    SchemaConflict                { conflict: String               },
    MissingSchemaField            { field:    String               },
    SchemaFieldTypeMismatch       { field: String, expected: String, actual: String },
    ColumnNotFound                { column:   String               },
    DuplicateColumn               { column:   String               },
    ColumnTypeMismatch            { column: String, expected: String, actual: String },
    ColumnOperationFailed         { column: String, reason: String },
    UnexpectedNulls               { column:   String               },
    EmptyColumn                   { column:   String               },
    InvalidColumnName             { column: String, reason: String },
    IncompatibleColumnShapes      { details:  String               },
    InvalidColumnValues           { column:String, invalid_count:usize, total_count:usize },
    EmptyDataFrame                {                             },
    DataFrameShapeMismatch        { expected_rows:usize, expected_cols:usize,
                                     actual_rows:usize,   actual_cols: usize },
    JoinFailed                    { reason:   String               },
    JoinKeyNotFound               { key:String, side:String      },
    JoinKeyTypeMismatch           { left_type:String, right_type:String },
    AggregationFailed             { column:String, reason:String },
    GroupByFailed                 { reason:   String               },
    WindowFunctionFailed          { function:String, reason:String },
    PivotFailed                   { reason:   String               },
    SortFailed                    { columns:String, reason:String },
    FilterFailed                  { reason:   String               },
    RowSelectionFailed            { reason:   String               },
    ConcatenationFailed           { reason:   String               },
    CastFailed                    { column:String, from_type:String, to_type:String, reason:String },
    UnsupportedDataType           { data_type:String, operation:String },
    TypeInferenceFailed           { column:String, reason:String },
    ValueConversionFailed         { value:String, target_type:String, reason:String },
    NumericOverflow               { column:String, details:String },
    DateTimeFormat                { column:String, expected:String, actual:String },
    StringEncoding                { column:String, details:String },
    ParquetReadFailed             { path:PathBuf, reason:String },
    ParquetWriteFailed            { path:PathBuf, reason:String },
    ParquetCorrupted              { path:PathBuf },
    ParquetSchemaIncompatible     { details:String },
    ParquetCompressionNotSupported{ compression:String },
    ParquetRowGroupInvalid        { row_group:usize, reason:String },
    ParquetStatisticsInvalid      { column:String },
    ParquetVersionUnsupported     { version:String },
    PipelineStepFailed            { step:String, reason:String },
    PipelineValidationFailed      { details:String },
    DataSourceUnavailable         { resource:String, reason:String },
    DataSinkFailed                { sink:String, reason:String },
    TransformationFailed          { transformation:String, reason:String },
    LineageTrackingFailed         { reason:String },
    BatchProcessingFailed         { batch_id:String, reason:String },
    StreamProcessingFailed        { reason:String },
    DependencyNotSatisfied        { dependency:String, reason:String },
    DataQualityFailed             { check:String, details:String },
    ConstraintViolation           { column:String, constraint:String },
    CompletenessCheckFailed       { column:String, missing_percentage:f64, threshold:f64 },
    UniquenessViolation           { column:String, duplicate_count:usize },
    RangeValidationFailed         { column:String, value:String, min:String, max:String },
    PatternValidationFailed       { column:String, value:String, pattern:String },
    ReferentialIntegrityViolation { details:String },
    DataFreshnessCheckFailed      { age_hours:u64, threshold_hours:u64 },
    LazyFrameExecutionFailed      { reason:String },
    QueryOptimizationFailed       { reason:String },
    LazyOperationNotSupported     { operation:String, reason:String },
    PredicatePushdownFailed       { reason:String },
    ProjectionPushdownFailed      { reason:String },
    QueryPlanFailed               { reason:String },
    OutOfMemory                   { operation:String, details:String },
    OperationTimeout              { operation:String, timeout_seconds:u64 },
    ResourceLimitExceeded         { limit_type:String, limit_value:String },
    ParallelThread                { details:String },
    PerformanceDegradation        { operation:String, actual_ms:u64, threshold_ms:u64 },
    InvalidConfiguration          { component:String, reason:String },
    MissingConfiguration          { key:String },
    ConnectionFailed              { resource:String },
    AuthenticationFailed          { resource:String, reason:String },
    PermissionDenied              { operation:String, resource:String },
    Environment                   { details:String },
    Multiple                       { count:usize, errors:Vec<Error> },
    Context                        { message:String },
    Polars                         { details:String },
    Arrow                          { details:String },
    ExtLib                         { library:String, details:String },
}
