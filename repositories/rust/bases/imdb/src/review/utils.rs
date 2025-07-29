use super::commands::{Commands::*, *};
use crate::{Datasets, data::Namespace};
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
  /// Use data namespace
  #[arg(long, global = true)]
  pub no_data_namespace: bool,

  /// Use imdb namespace
  #[arg(long, global = true)]
  pub no_imdb_namespace: bool,

  /// Disable namespacing
  #[arg(long, global = true)]
  pub no_namespacing: bool,

  /// Accept multiple namespaces, up to 2
  #[arg(short, long, global = true, num_args = 1..=2)]
  pub namespace: Vec<String>,

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

  debug!("{cli:#?}");

  // Normalize namespace vector into Namespace struct
  let namespace =
    if cli.no_namespacing || (cli.no_data_namespace && cli.no_imdb_namespace) {
      Namespace::flat()
    } else if !cli.namespace.is_empty() {
      match cli.namespace.len() {
        1 => Namespace::single(cli.namespace[0].clone()),
        2 => {
          Namespace::nested(cli.namespace[0].clone(), cli.namespace[1].clone())
        }
        _ => unreachable!("clap limits to max 2 namespaces"),
      }
    } else if !cli.no_data_namespace {
      Namespace::no_data()
    } else if !cli.no_imdb_namespace {
      Namespace::no_imdb()
    } else {
      Namespace::default()
    };

  debug!("{namespace:#?}");
  let temp_path = env!("CARGO_MANIFEST_DIR");
  debug!("{temp_path:#?}");
  debug!("{:#?}", namespace.path(temp_path));

  // let datasets = Datasets::new(cli.data_namespace, cli.imdb_namespace);
  // let force_action = cli.force;

  // match cli.command.unwrap_or(Check {}) {
  //   Reset {} => {
  //     purge(&datasets);
  //     download(&datasets, force_action).await?;
  //     extract(&datasets, force_action);
  //     clean(&datasets, force_action);
  //   }
  //   Download {} => {
  //     download(&datasets, force_action).await?;
  //     // extract(&datasets, force_action);
  //     // clean(&datasets, force_action);
  //   }
  //   Extract {} => {
  //     extract(&datasets, force_action);
  //     clean(&datasets, force_action);
  //   }
  //   Clean {} => clean(&datasets, force_action),
  //   Check {} => check(&datasets)
  // }

  Ok(())
}
