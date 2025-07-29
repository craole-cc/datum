// -- Application (main.rs) -- //
use imdb::*;

// region: Main Execution
#[tokio::main]
async fn main() -> Result<()> {
  println!("    ┌────────────────────────────────────────────────────┐");
  println!(
    "    │ Welcome to the {} ({} v.{})",
    env!("CARGO_PKG_DESCRIPTION"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
  );
  println!("    └────────────────────────────────────────────────────┘");

  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::TRACE)
    .without_time()
    .with_target(false)
    .init();

  imdb::utils::inspect_file_manually(Path::new("/data/test.tsv"))?;
  // imdb_cli::run().await?; // TODO: Get instructions from the cli to update the config and proceed
  // imdb::init()?;

  Ok(())
}
// endregion

// -- End of the Application module (main.rs) -- //

// if let Err(e) = ingest::run() {
//   error!("Data ingestion failed: {e}");
//   return Err(e);
// }
