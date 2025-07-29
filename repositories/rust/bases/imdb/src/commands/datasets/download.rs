use super::*;

pub fn execute(force: bool) -> Result<()> {
  if force {
    println!("Downloading with force");
  } else {
    println!("Downloading");
  }
  Ok(())
}
