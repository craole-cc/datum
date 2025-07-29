// -- Cli Commands (commands/mod.rs) -- //

// region: Module Imports
pub mod datasets;
// endregion

// region: Internal API
use super::*;
use crate::Result;
// endregion

// region: Module Exports

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Manage datasets
  #[clap(alias = "dataset", alias = "data")]
  Datasets {
    #[command(subcommand)]
    command: datasets::Commands,
  },
}

/// Handles the top-level commands.
pub fn handle_command(command: Commands, force: bool) -> Result<()> {
  match command {
    Commands::Datasets { command } => {
      datasets::handle_command(command, force)?;
    }
  }
  Ok(())
}

// endregion

// -- End of the Cli Commands module -- //
