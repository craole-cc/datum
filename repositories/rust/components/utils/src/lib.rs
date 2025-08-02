mod examples;
mod timer;
pub use timer::*;

// mod scope;
// pub use scope::*;

pub fn print_banner(description: &str, name: &str, version: &str) {
  // Build the core message and compute its length
  let msg = format!("Welcome to the {description} ({name} v.{version})");
  let width = msg.chars().count();

  // Define border pieces
  let indent = "    ";
  let horiz = "─".repeat(width + 2); // +2 for a space on each side of the message

  // Top border
  println!("{indent}┌{horiz}┐");

  // Message line, padded by one space each side
  println!("{indent}│ {msg} │");

  // Bottom border
  println!("{indent}└{horiz}┘");
}

pub fn init_tracing(lvl: tracing::Level) {
  tracing_subscriber::fmt()
    .with_max_level(lvl)
    .without_time()
    .with_target(false)
    .init();
}
