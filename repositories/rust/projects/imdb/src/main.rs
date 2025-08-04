// -- Application (main.rs) -- //
use erks::*; // Errors and Tracing
use utils::*; // Reuable functions

#[tokio::main]
async fn main() -> Result<()> {
  let desc = env!("CARGO_PKG_DESCRIPTION");
  let name = env!("CARGO_PKG_NAME");
  let vr3n = env!("CARGO_PKG_VERSION");
  print_banner(desc, name, vr3n);
  set_tracing_debug();
  set_panic_hook();

  // match imdb_cli::run().await {
  //   Ok(_) => {}
  //   Err(e) => eprintln!("{e:?}"),
  // }

  // match imdb::config::init() {
  //   Ok(_) => {}
  //   Err(e) => eprintln!("{e:?}"),
  // }
  // bail!(imdb::config::init())?;
  // if vr3n != "pop" {
  //   bail!("permission denied for accessing {vr3n}");
  // }
  imdb_cli::run().await;
  imdb::config::init()
    .wrap_err("Encountered issues initializing the config")?;
  // imdb::config::init().bail_with_context("Config initialization failed")?;
  // imdb::config::init().log_error_with_context("Static string".to_string());
  // imdb::config::init()
  //   .log_error_with_context(format!("Config failed at {}", line!()));

  Ok(())
}

// -- End of the Application module (main.rs) -- //
