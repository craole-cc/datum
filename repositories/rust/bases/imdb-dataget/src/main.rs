#[macro_use]
extern crate tracing;

mod cli;
mod data;

pub use data::Datasets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .with_target(false)
    .init();

  cli::run().await;
  Ok(())
}
