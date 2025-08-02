// src/via_polars/mod.rs

mod bronze;

use crate::*;
use polars::prelude::*;

pub async fn execute() -> TheResult<()> {
  bronze::execute().await?;
  Ok(())
}
