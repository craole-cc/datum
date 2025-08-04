use crate::*;
mod bronze;
pub async fn execute() -> Result<()> {
  let imdb = Config::default();
  trace!("IMDB {imdb:#?}");
  bronze::execute(&imdb).await?;

  Ok(())
}
