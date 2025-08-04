use crate::*;

mod ingestion;

#[derive(Default, Debug)]
pub struct Config {
  pub datasets: imdb_dataset::Datasets,
  pub ingestion: ingestion::Config,
}
