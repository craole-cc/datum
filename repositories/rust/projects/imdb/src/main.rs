// -- Application (main.rs) -- //

use erks::{Context, trace_call};

#[tokio::main]
async fn main() -> erks::Result<()> {
  let desc = env!("CARGO_PKG_DESCRIPTION");
  let name = env!("CARGO_PKG_NAME");
  let vr3n = env!("CARGO_PKG_VERSION");
  util::print_banner(desc, name, vr3n);

  erks::set_tracing_trace();
  erks::set_panic_hook();

  // imdb_cli::run().await;
  // trace_call!(
  //   imdb::pipeline::execute().await?;
  // )
  match trace_call!(
    imdb::pipeline::execute().await.wrap_err("Pipeline failure")
  ) {
    Ok(_) => println!("Success"),
    Err(e) => eprintln!("Error: {e:?}"),
  }
  Ok(())
}
// -- End of the Application module (main.rs) -- //
