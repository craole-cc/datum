// -- Dataset Commands (commands/datasets/mod.rs) -- //

// region: Module Imports
mod check;
mod download;
mod extract;
mod ingest;
mod purge;
// endregion

// region: Internal API
use super::*;
// endregion

// region: Module Exports

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Check if all datasets exist and are valid
  Check {},
  /// Download all datasets and prepare for import
  #[clap(alias = "fetch", alias = "update", alias = "up")]
  Download {},
  /// Extract all datasets
  Extract {},
  /// Remove extracted files to prepare for import
  Ingest {},
  /// Reset data, remove all downloads and extracted files
  Reset {},
}

/// Handles the datasets subcommands.
pub fn handle_command(command: Commands, force: bool) -> Result<()> {
  match command {
    Commands::Check {} => check::execute()?,
    Commands::Download {} => download::execute(force)?,
    Commands::Extract {} => extract::execute(force)?,
    Commands::Ingest {} => ingest::execute(force)?,
    Commands::Reset {} => purge::execute(force)?,
  }

  Ok(())
}

// endregion

// -- End of the Cli Commands module -- //
