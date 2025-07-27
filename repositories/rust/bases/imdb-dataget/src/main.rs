// -- Module Imports -- //
// mod cli;
mod data;
mod utils;

// -- Local Exports -- //
#[macro_use]
extern crate tracing;
use anyhow::{Context, Result};
use data::Datasets;
use std::path::PathBuf;
use utils::*;

// -- External Exports -- //

// -- Main Execution -- //
#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .without_time()
    .with_target(false)
    .init();

  let mut home = data::Home::default();
  let datasets = Datasets::default();
  // home.with_base("pop").with_category("lol");
  debug!("{:#?}", home.to_pathbuf());
  debug!(
    "{}",
    crate::utils::filename_from_url(
      "https://datasets.imdbws.com/title.basics.tsv.gz"
    )?
  );
  debug!("{:#?}", datasets);
  // cli::run().await?;
  Ok(())
}
