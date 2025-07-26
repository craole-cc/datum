use crate::Datasets;
use anyhow::{Context, Result};
use downloader::Downloader;
use std::{fs, path::PathBuf};
use tokio::fs as async_fs;

pub async fn download(datasets: &Datasets, force: bool) -> Result<()> {
  let urls: Vec<_> = datasets
    .all()
    .iter()
    .map(|dataset| dataset.url.clone())
    .collect();
  let concurrency_limit = (urls.len() as f32 / 1.7).round() as usize;
  let home = datasets.profiles.source.parent().unwrap();
  let mut download = Downloader::new(urls, home, Some(concurrency_limit));
  // download.check_existing_files().await?;
  // match download {
  //   Ok(existing) => {
  //     if !existing.is_empty() {
  //       println!("The following files already exist:");
  //       for (url, path) in existing {
  //         println!("-> {url:#?} \n-> {path:#?}");
  //       }
  //       println!("Use --force to overwrite existing files");
  //       return Ok(());
  //     }
  //   }
  //   Err(e) => return Err(e.into()),
  // }
  download.start(true).await?;
  // eprintln!("Downloading {} files to {home:#?}", urls.len());
  // eprintln!("{urls:#?}");
  // eprintln!("{download:#?}");
  // for dataset in datasets.all() {
  //   let target_path = dataset.source.clone();
  //   let urls = vec![dataset.url.clone()];

  //   if target_path.exists() && !force {
  //     println!("File {target_path:?} exists; skipping");
  //     continue;
  //   }

  //   println!(
  //     "Downloading \n  URL: {} \n  SRC: {:#?}",
  //     dataset.url, target_path
  //   );

  //   let tmp_path = target_path.with_extension("tmp");
  //   // let downloader = Downloader::new(&dataset.url, &tmp_path);
  // }
  // Iterate all datasets (assuming you add a helper to get them as slice or vector)
  // for ds in datasets.all() {
  //   let target_path: PathBuf = ds.source.clone();

  //   if target_path.exists() && !force {
  //     println!("File {} exists; skipping", target_path.display());
  //     continue;
  //   }

  //   println!("Downloading {} to {}", ds.url, target_path.display());

  //   let downloader = Downloader::new(&ds.url, &target_path);

  //   downloader
  //     .download()
  //     .await
  //     .with_context(|| format!("Failed to download {}", ds.name))?;
  // }

  // println!("All downloads completed.");
  Ok(())
}
