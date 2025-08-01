mod bronze;

use crate::*;
use deltalake::datafusion::{
  arrow::array::{Int32Array, RecordBatch, StringArray},
  arrow::datatypes::{DataType, Field, Schema},
  dataframe::DataFrameWriteOptions,
  error::Result,
  functions_aggregate::expr_fn::min,
  prelude::*,
};

pub async fn execute() -> TheResult<()> {
  bronze::execute().await?;
  Ok(())
}
