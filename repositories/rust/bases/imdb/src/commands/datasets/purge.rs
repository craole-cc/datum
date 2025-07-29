use super::*;

pub fn execute(force: bool) -> Result<()> {
  if force {
    println!("Purging with force");
  } else {
    println!("Purging");
  }
  Ok(())
}
