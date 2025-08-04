use super::*;
use deltalake::{
  DeltaTableBuilder,
  arrow::datatypes::{DataType, Field, Schema},
  datafusion::{
    config::CsvOptions,
    datasource::file_format::csv::CsvFormat,
    logical_expr::ExprSchemable,
    prelude::{SessionContext, col, lit, when},
  },
  writer::{DeltaWriter, RecordBatchWriter},
};
use imdb_dataset::Dataset;

mod credits;
mod crews;
mod profiles;
mod ratings;
mod series;
mod titles;
mod variants;

const LAYER: &str = "BRONZE";

pub async fn execute(cfg: &Config) -> Result<()> {
  let ctx = SessionContext::new();
  titles::execute(&ctx, cfg).await?;
  Ok(())
}
