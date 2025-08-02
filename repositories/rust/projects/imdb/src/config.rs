use crate::*;

#[derive(Default, Debug)]
pub struct Config {
  pub datasets: imdb_dataset::Datasets,
  // pub ingestion: ingestion::Config,
}

pub fn init() -> Result<()> {
  let imdb = &mut Config::default();
  debug!("Initialized config {imdb:#?}");
  // ingestion::execute(imdb);
  Ok(())
}
