use crate::preview::DownloadPreview;
use crate::{Error, Result};
use std::io::{self, Write};
use std::path::PathBuf;

/// Options for user interaction during preview
#[derive(Debug, Clone, PartialEq)]
pub enum PreviewAction {
  /// Proceed with all downloads
  ProceedAll,
  /// Skip specific downloads by index
  ProceedSelected(Vec<usize>),
  /// Cancel all downloads
  Cancel,
  /// Show detailed information
  ShowDetails,
}

/// Trait for user interface implementations
#[async_trait::async_trait]
pub trait UserInterface {
  /// Shows the preview and gets user choice
  async fn show_preview_and_get_choice(
    &self,
    previews: &[DownloadPreview],
    home: &PathBuf,
    concurrency_limit: Option<usize>,
  ) -> Result<PreviewAction>;

  /// Shows detailed preview information
  async fn show_detailed_preview(&self, previews: &[DownloadPreview]);

  /// Shows a message to the user
  async fn show_message(&self, message: &str);
}

/// Console-based user interface implementation
#[derive(Debug, Default)]
pub struct ConsoleInterface;

impl ConsoleInterface {
  /// Creates a new ConsoleInterface
  pub fn new() -> Self {
    Self::default()
  }

  /// Displays a basic preview of the downloads
  fn display_preview(
    &self,
    previews: &[DownloadPreview],
    home: &PathBuf,
    concurrency_limit: Option<usize>,
  ) {
    println!("\n=== Download Preview ===");
    println!("Destination directory: {}", home.display());
    println!("Number of files: {}", previews.len());
    println!("Concurrency limit: {:?}", concurrency_limit);
    println!("\nFiles to download:");

    for preview in previews {
      println!("{}", preview.display());
    }
    println!();
  }

  /// Gets user choice for how to proceed with downloads
  async fn get_user_choice(
    &self,
    previews: &[DownloadPreview],
  ) -> Result<PreviewAction> {
    loop {
      println!("What would you like to do?");
      println!("  [a] Download all files");
      println!("  [s] Select specific files to download");
      println!("  [d] Show detailed information");
      println!("  [c] Cancel");
      print!("Choice (a/s/d/c): ");
      io::stdout().flush().unwrap();

      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      let choice = input.trim().to_lowercase();

      match choice.as_str() {
        "a" | "all" => return Ok(PreviewAction::ProceedAll),
        "c" | "cancel" => return Ok(PreviewAction::Cancel),
        "d" | "details" => return Ok(PreviewAction::ShowDetails),
        "s" | "select" => match self.get_file_selection(previews) {
          Ok(indices) if !indices.is_empty() => {
            return Ok(PreviewAction::ProceedSelected(indices));
          }
          Ok(_) => {
            println!("No files selected. Please try again.\n");
            continue;
          }
          Err(e) => {
            println!("Invalid selection: {}. Please try again.\n", e);
            continue;
          }
        },
        _ => {
          println!("Invalid choice. Please enter 'a', 's', 'd', or 'c'.\n");
          continue;
        }
      }
    }
  }

  /// Gets user selection of specific files to download
  fn get_file_selection(
    &self,
    previews: &[DownloadPreview],
  ) -> Result<Vec<usize>> {
    println!(
      "\nSelect files to download (enter numbers separated by spaces or commas):"
    );
    println!("Example: 1,3,5 or 1 3 5");
    print!("Selection: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut indices = Vec::new();
    let cleaned_input = input.trim().replace(',', " ");

    for part in cleaned_input.split_whitespace() {
      match part.parse::<usize>() {
        Ok(num) if num > 0 && num <= previews.len() => {
          indices.push(num - 1); // Convert to 0-based indexing
        }
        Ok(num) => {
          return Err(Error::invalid_url(&format!(
            "File number {} is out of range (1-{})",
            num,
            previews.len()
          )));
        }
        Err(_) => {
          return Err(Error::invalid_url(&format!("Invalid number: {}", part)));
        }
      }
    }

    // Remove duplicates and sort
    indices.sort_unstable();
    indices.dedup();

    Ok(indices)
  }
}

#[async_trait::async_trait]
impl UserInterface for ConsoleInterface {
  async fn show_preview_and_get_choice(
    &self,
    previews: &[DownloadPreview],
    home: &PathBuf,
    concurrency_limit: Option<usize>,
  ) -> Result<PreviewAction> {
    self.display_preview(previews, home, concurrency_limit);
    self.get_user_choice(previews).await
  }

  async fn show_detailed_preview(&self, previews: &[DownloadPreview]) {
    println!("\n=== Detailed Download Information ===");

    for preview in previews {
      println!("{}", preview.detailed_info());
      println!();
    }
  }

  async fn show_message(&self, message: &str) {
    println!("{}", message);
  }
}
