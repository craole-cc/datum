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
