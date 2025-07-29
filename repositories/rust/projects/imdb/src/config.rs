use crate::*;

#[derive(Default, Debug)]
pub struct Config {
  pub datasets: Datasets,
  pub ingestion: ingestion::Config,
}

pub fn init() -> Result<()> {
  let imdb = &mut Config::default();
  // ingestion::execute(imdb);
  Ok(())
}
