use super::commands::{Commands::*, *};
use crate::{Datasets, cli, data};
use anyhow::Result;
use clap::Parser;
use downloader::Downloader;

#[derive(Parser, Debug)]
pub struct Cli {
  /// Base data directory overrides default location
  #[arg(short, long, global = true)]
  pub data_namespace: bool,

  /// Base data namespace creates a subdirectory within the data directory
  #[arg(short, long, global = true)]
  pub imdb_namespace: bool,

  /// Force overwrite existing files without prompting
  #[arg(short, long, global = true)]
  pub force: bool,

  #[command(subcommand)]
  pub command: Option<Commands>,
}

pub async fn run() -> Result<()> {
  println!(
    "Welcome to imdb-dataget: a simple CLI for fetching and prepping IMDb datasets!"
  );

  let cli = Cli::parse();
  let datasets = Datasets::new(cli.data_namespace, cli.imdb_namespace);
  let force_action = cli.force;

  match cli.command.unwrap_or(Check {}) {
    Reset {} => {
      purge(&datasets);
      download(&datasets, force_action);
      extract(&datasets, force_action);
      clean(&datasets, force_action);
    }
    Download {} => {
      println!("Downloading datasets...");
      download(&datasets, force_action).await?;
      // extract(&datasets, force_action);
      // clean(&datasets, force_action);
    }
    Extract {} => {
      extract(&datasets, force_action);
      clean(&datasets, force_action);
    }
    Clean {} => clean(&datasets, force_action),
    Check {} => check(&datasets),
  }

  Ok(())
}
