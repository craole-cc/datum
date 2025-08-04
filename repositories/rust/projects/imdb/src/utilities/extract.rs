use super::*;

// pub async fn dataset_from_source(layer: &str, dataset: &str) -> TheResult<()> {
//   let mut scope = Scope::new(layer, dataset);

//   let config: Config = scope
//     .time_async("Dataset Validation", || async move {
//       let cfg = Config::new(dataset).wrap_err_with(|| {
//         format!("Failed to create config for dataset: {dataset}")
//       })?;

//       cfg.ensure_source_exists().wrap_err_with(|| {
//         format!("Source validation failed for dataset: {dataset}")
//       })?;

//       debug!("{:#?}", cfg);
//       Ok(cfg)
//     })
//     .await?;

//   let frame = scope
//     .time_async("Source Ingestion", || async {
//       let df = ingest::source_to_frame(&config).await.wrap_err_with(|| {
//         format!("CSV ingestion failed for dataset: {dataset}")
//       })?;

//       // Test DataFrame validity - convert DataFusion error to miette error
//       let count = df
//         .clone()
//         .count()
//         .await
//         .map_err(|e| miette!(
//           help = "This error occurred while validating the ingested DataFrame at extract.rs:{}
// • Check CSV file integrity (look for missing fields or malformed rows)
// • Verify delimiter configuration matches your data
// • Inspect the problematic line mentioned in the error above",
//           "DataFrame row count validation failed for dataset '{}' (extract.rs:{}): {}",
//           dataset,
//           line!(),
//           e
//         ))?;

//       info!(
//         "Successfully loaded {} rows for dataset: {}",
//         count, dataset
//       );
//       Ok(df)
//     })
//     .await?;

//   let parquet = scope
//     .time_async("Source Conversion", || async {
//       export::frame_to_parquet(&config, frame)
//         .await
//         .wrap_err_with(|| {
//           format!("Parquet conversion failed for dataset: {dataset}")
//         })
//     })
//     .await?;

//   info!(%dataset, %layer, "Dataset extraction successful");
//   Ok(())
// }

pub async fn dataset_from_source(layer: &str, dataset: &str) -> TheResult<()> {
  let mut scope = Scope::new(layer, dataset);

  let result: TheResult<()> = async {
    let config: Config = scope
      .time_async("Dataset Validation", || async move {
        let cfg = Config::new(dataset)
          .map_err(|e| miette!(
            help = "Check if the dataset name exists in your configuration mappings",
            "Failed to create configuration for dataset '{}'",
            dataset
          ).wrap_err(e))?;

        cfg.ensure_source_exists()
          .map_err(|e| miette!(
            help = "Verify the source file path and ensure the file exists and is readable",
            "Source file validation failed for dataset '{}'",
            dataset
          ).wrap_err(e))?;

        debug!("{:#?}", cfg);
        Ok(cfg)
      })
      .await?;
    let frame = scope
    .time_async("Source Ingestion", || async {
      let df = ingest::source_to_frame(&config)
      .await
      .map_err(|e| miette!(
        help = "Common CSV ingestion issues:
        • Check delimiter configuration (tab vs comma)
        • Verify null value patterns match your data
        • Look for malformed CSV lines or embedded newlines
        • Ensure file encoding is UTF-8",
        "Failed to read CSV data for dataset '{}'",
        dataset
      ).wrap_err(e))?;

      error!("HERE");
      // Test if the DataFrame is actually readable
      match df.clone().count().await {
        Ok(count) => {
          info!("Successfully loaded {} rows for dataset: {}", count, dataset);
        }
        Err(e) => {
          return Err(miette!(
            help = "The CSV was partially read but contains data integrity issues:
            • Check for inconsistent field counts across rows
            • Look for unescaped quotes or special characters
            • Verify the file isn't truncated or corrupted",
            "DataFrame validation failed after ingestion for dataset '{dataset}')"
          ).wrap_err(e));
        }
      }
//             // Test DataFrame validity - convert DataFusion error to miette error
//       let count = df
//         .clone()
//         .count()
//         .await
//         .map_err(|e| miette!(
//           help = "This error occurred while validating the ingested DataFrame at extract.rs:{}
// • Check CSV file integrity (look for missing fields or malformed rows)
// • Verify delimiter configuration matches your data
// • Inspect the problematic line mentioned in the error above",
//           "DataFrame row count validation failed for dataset '{}' (extract.rs:{}): {}",
//           dataset,
//           line!(),
//           e
//         ))?;

//       info!(
//         "Successfully loaded {} rows for dataset: {}",
//         count, dataset
//       );
      error!("After");

        // Only try to show preview if count succeeded
        match df.clone().show_limit(5).await {
          Ok(preview) => {
            debug!("DataFrame preview for {}:\n{:#?}", dataset, preview);
          }
          Err(e) => {
            warn!("Could not preview DataFrame for {}: {:?}", dataset, e);
          }
        }

        Ok(df)
      })
      .await?;

    let parquet = scope
      .time_async("Source Conversion", || async {
        export::frame_to_parquet(&config, frame)
          .await
          .map_err(|e| miette!(
            help = "Parquet conversion issues:
• Check available disk space
• Verify write permissions to output directory
• Data type compatibility issues between DataFrame and Parquet
• Memory constraints for large datasets",
            "Failed to convert DataFrame to Parquet for dataset '{}'",
            dataset
          ).wrap_err(e))
      })
      .await?;

    Ok(())
  }
  .await;

  match result {
    Ok(_) => {
      info!(%dataset, %layer, "Dataset extraction successful");
    }
    Err(err) => {
      return Err(miette!(
        help = "Dataset extraction failed. Check the specific error above for detailed guidance.",
        "{layer} layer processing failed for dataset '{dataset}'"
      ).wrap_err(err));
    }
  }

  Ok(())
}
