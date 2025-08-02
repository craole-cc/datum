// -- Application (main.rs) -- //

// region: Main Execution
#[tokio::main]
async fn main() -> erks::Result<()> {
  let desc = env!("CARGO_PKG_DESCRIPTION");
  let name = env!("CARGO_PKG_NAME");
  let vr3n = env!("CARGO_PKG_VERSION");
  utils::print_banner(desc, name, vr3n);
  erks::set_tracing_debug();

  // imdb_cli::run().await?; // TODO: Get instructions from the cli to update the config and proceed
  imdb::config::init();
  Ok(())
}
// endregion

// -- End of the Application module (main.rs) -- //
