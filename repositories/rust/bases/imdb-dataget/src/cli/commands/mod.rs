mod check;
mod clean;
mod download;
mod extract;
mod purge;

pub use check::*;
pub use clean::*;
pub use download::*;
pub use extract::*;
pub use purge::*;

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
  /// Check if all datasets exist and are valid
  Check {},
  /// Download all datasets and prepare for import
  #[clap(alias = "fetch", alias = "update", alias = "up")]
  Download {},
  /// Extract all datasets
  Extract {},
  /// Remove extracted files to prepare for import
  Clean {},
  /// Reset data, remove all downloads and extracted files
  Reset {}
}
