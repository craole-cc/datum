use super::*;

pub fn execute(force: bool) -> Result<()> {
  if force {
    println!("Extracting with force");
  } else {
    println!("Extracting");
  }
  Ok(())
}
