use crate::*;
use anyhow::{Context, Result};
use downloader::Downloader;

pub async fn download(datasets: &Datasets, force: bool) -> Result<()> {
  let home = datasets
    .profiles
    .source
    .parent()
    .with_context(|| {
      format!("Invalid source path `{:?}`", datasets.profiles.source)
    })?
    .to_path_buf();
  let urls: Vec<_> = datasets
    .all()
    .iter()
    .map(|dataset| dataset.url.clone())
    .collect();
  let mut dldr = Downloader::new(urls, home)?;
  dldr.with_concurrency_limit(4);
  if force {
    dldr.overwrite_existing();
  } else {
    dldr.skip_existing();
  }
  debug!("{dldr:#?}");
  // let preview = downloader.preview().await?;

  // This will now block until all downloads complete
  // let _progress_reporter = downloader.start().await?;

  Ok(())
}
