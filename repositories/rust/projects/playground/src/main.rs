use playground::*;
mod test_miette;
#[tokio::main]
async fn main() -> TheResult<()> {
  let desc = env!("CARGO_PKG_DESCRIPTION");
  let name = env!("CARGO_PKG_NAME");
  let vr3n = env!("CARGO_PKG_VERSION");
  print_banner(desc, name, vr3n);

  init_tracing(tracing::Level::INFO);

  configure_timing(TimingConfig {
    print_immediately: false,
    print_summary: false,
    ..Default::default()
  });

  // bail!(
  //   "Database connection timeout after 30 seconds\n({}:{})",
  //   file!(),
  //   line!()
  // );
  // via_deltalake::execute().await?;
  println!("->> Hello world!");
  test_miette::main().await?;
  Ok(())
}
