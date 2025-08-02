use super::*;
const LAYER: &str = "Bronze";

pub async fn execute(datasets_to_debug: &[&str]) -> TheResult<()> {
  let datasets_to_process: Vec<_> = if datasets_to_debug.is_empty() {
    DATASETS.iter().collect()
  } else {
    let debug_set: HashSet<String> = datasets_to_debug
      .iter()
      .map(|name| name.to_lowercase())
      .collect();

    DATASETS
      .iter()
      .filter(|(display_name, _)| {
        debug_set.contains(&display_name.to_lowercase())
      })
      .collect()
  };

  // Early return if debug datasets are specified but none are found
  if !datasets_to_debug.is_empty() && datasets_to_process.is_empty() {
    warn!(%LAYER, "No matching debug datasets found");
    return Ok(()); // Graceful exit for debug mode
  } else {
    info!(%LAYER, "Processing datasets: {:?}",datasets_to_process);
  }

  // Process the selected datasets in parallel
  let futures = datasets_to_process
    .iter()
    .map(|(display_name, _)| extract::dataset_from_source(LAYER, display_name));
  let results = join_all(futures).await;

  // Match results back to their display names
  for ((display_name, _), result) in DATASETS.iter().zip(results) {
    result?
  }

  Ok(())
}
