use crate::*;
use commands::*;

#[derive(Parser, Debug)]
pub struct Cli {
  /// Force overwrite existing files without prompting
  #[arg(short, long, global = true)]
  pub force: bool,

  #[command(subcommand)]
  pub command: Option<Commands>,
}

pub async fn run() -> Result<()> {
  let mut cli = Cli::parse();
  let force = cli.force;

  //~@ Set a default command if none is provided
  if cli.command.is_none() {
    cli.command = Some(Commands::Datasets {
      command: datasets::Commands::Check {},
    });
  }
  trace!("{cli:#?}");

  if let Some(command) = cli.command {
    commands::handle_command(command, force)?;
  }

  Ok(())
}
