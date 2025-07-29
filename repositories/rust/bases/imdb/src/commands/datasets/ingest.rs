use super::*;

pub fn execute(force: bool) -> Result<()> {
  if force {
    println!("Ingesting with force");
  } else {
    println!("Ingesting");
  }
  Ok(())
}
