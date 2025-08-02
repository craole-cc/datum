use crate::{via_deltalake::utilities::*, *};

mod bronze;
mod silver;

pub async fn execute() -> TheResult<()> {
  bronze::execute(&["Title"]).await?;
  Ok(())
}
