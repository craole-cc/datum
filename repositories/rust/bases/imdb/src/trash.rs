// -- Module Imports -- //
// mod cli;
// mod data;
// mod utils;

// -- Local Exports -- //
#[macro_use]
extern crate tracing;
use anyhow::{Context, Result};
use std::path::PathBuf;

// -- External Exports -- //

// -- Main Execution -- //
#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .without_time()
    .with_target(false)
    .init();

  // let imdb = Datasets::default();
  // info!("IMDB datasets: {imdb:#?}");
  // match imdb.file_info() {
  //   Ok(inf) => {
  //     if !inf.download_exists {
  //       // TODO: Use the downloader to pull the download
  //       // TODO: Use the extractor to extract the data to the same directory
  //       debug!("download does not exist, downloading");
  //     } else if !inf.import_exists {
  //       // TODO: Check if the file has been extracted (.tsv exists)
  //       // TODO: Clean the extracted data and move it to import
  //       debug!(
  //         "Import does not exist. We should extract, clean and move the data."
  //       );
  //     } else {
  //       debug!("Import exists and should already be clean");
  //     }
  //   }
  //   Err(e) => {
  //     debug!("No file info: {}", e);
  //   }
  // }
  // cli::run().await?;
  Ok(())
}
