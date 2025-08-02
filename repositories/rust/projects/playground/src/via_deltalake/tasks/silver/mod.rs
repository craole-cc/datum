use super::*;

pub async fn initialize_dataset(layer: &str, dataset: &str) -> TheResult<()> {
  let config = Config::new(dataset)?;
  let path = config.import_path();
  let path_str = &path.to_string_lossy();

  let df = ingest::parquet_to_frame(path_str).await?;

  info!(
    "Loaded {} with {} columns",
    dataset,
    df.schema().fields().len()
  );

  Ok(())
}
