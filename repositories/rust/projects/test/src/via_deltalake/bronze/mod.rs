use super::*;

/// Execute the bronze layer processing - convert all TSV files to Parquet
pub async fn execute() -> TheResult<()> {
  println!("🏗️  Starting Bronze Layer Processing...");

  // Create all dataset configs
  let datasets = vec![
    ("credits", Config::new("credits")?),
    ("crew", Config::new("crew")?),
    ("profiles", Config::new("profiles")?),
    ("ratings", Config::new("ratings")?),
    ("series", Config::new("series")?),
    ("title", Config::new("title")?),
    ("variants", Config::new("variants")?),
  ];

  // Process each dataset
  let mut results = Vec::new();
  for (name, config) in datasets {
    println!("📊 Processing dataset: {name}");

    match process_dataset(&config).await {
      Ok(_) => {
        println!("✅ Successfully processed: {name}");
        results.push((name.to_string(), true));
      }
      Err(e) => {
        eprintln!("❌ Failed to process {name}: {e}");
        results.push((name.to_string(), false));
      }
    }
  }

  // Print summary
  print_summary(&results);

  Ok(())
}

/// Process a single dataset: TSV -> Parquet
async fn process_dataset(config: &Config) -> TheResult<()> {
  // Ensure source file exists
  if !config.source_exists() {
    anyhow::bail!("Source file does not exist: {}", config.path_as_str()?);
  }

  // Ensure import directory exists
  config.ensure_import_dir()?;

  // Get import path
  let import_path = config.import_path();

  println!("  📁 Source: {}", config.path_as_str()?);
  println!("  📁 Target: {}", import_path.display());
  println!("  📏 File size: {} MB", config.file_size()? / 1_024 / 1_024);

  // Check if target already exists
  if import_path.exists() {
    println!("  ⚠️  Target file already exists, overwriting...");
    remove_file(&import_path)
      .context("Failed to remove existing target file")?;
  }

  // Convert TSV to Parquet using DataFusion
  tsv_to_parquet(config, &import_path).await?;

  // Verify the output
  if import_path.exists() {
    let output_size = metadata(&import_path)?.len();
    println!("  ✨ Output size: {} MB", output_size / 1_024 / 1_024);
  }

  Ok(())
}

/// Convert TSV file to Parquet using DataFusion
async fn tsv_to_parquet(
  config: &Config,
  output_path: &std::path::Path,
) -> TheResult<()> {
  println!("  🔄 Converting TSV to Parquet...");

  let ctx = SessionContext::new();

  // Configure CSV reading options
  let csv_options = CsvReadOptions::new()
    .delimiter(config.delimiter)
    .file_extension(&config.source_ext)
    .has_header(true) // Most IMDB files have headers
    .schema_infer_max_records(1000); // Infer schema from first 1000 rows

  // Read the TSV file
  let df = ctx
    .read_csv(config.path_as_str()?, csv_options)
    .await
    .context("Failed to read TSV file")?;

  // Optional: Apply any bronze-layer transformations here
  let df = apply_bronze_transformations(df, config).await?;

  // Write to Parquet
  df.write_parquet(
    output_path
      .to_str()
      .context("Failed to convert output path to string")?,
    DataFrameWriteOptions::new(),
    None, // Use default parquet options
  )
  .await
  .context("Failed to write Parquet file")?;

  println!("  ✅ Conversion complete");
  Ok(())
}

/// Apply bronze layer transformations (basic cleanup, type inference, etc.)
async fn apply_bronze_transformations(
  df: DataFrame,
  config: &Config,
) -> TheResult<DataFrame> {
  // For bronze layer, we typically do minimal transformations
  // Just basic cleanup and standardization

  match config.name.as_str() {
    "ratings" => apply_ratings_transformations(df).await,
    "profiles" => apply_profiles_transformations(df).await,
    "title" => apply_title_transformations(df).await,
    "crew" => apply_crew_transformations(df).await,
    "credits" => apply_credits_transformations(df).await,
    "series" => apply_series_transformations(df).await,
    "variants" => apply_variants_transformations(df).await,
    _ => Ok(df), // No specific transformations
  }
}

/// Apply transformations specific to ratings dataset
async fn apply_ratings_transformations(df: DataFrame) -> TheResult<DataFrame> {
  // Example: Ensure rating values are within expected range
  // For bronze layer, we might just want to preserve raw data
  Ok(df)
}

/// Apply transformations specific to profiles dataset
async fn apply_profiles_transformations(df: DataFrame) -> TheResult<DataFrame> {
  // Example: Handle null birth/death years
  Ok(df)
}

/// Apply transformations specific to title dataset
async fn apply_title_transformations(df: DataFrame) -> TheResult<DataFrame> {
  // Example: Parse genres, handle runtime values
  Ok(df)
}

/// Apply transformations specific to crew dataset
async fn apply_crew_transformations(df: DataFrame) -> TheResult<DataFrame> {
  Ok(df)
}

/// Apply transformations specific to credits dataset
async fn apply_credits_transformations(df: DataFrame) -> TheResult<DataFrame> {
  Ok(df)
}

/// Apply transformations specific to series dataset
async fn apply_series_transformations(df: DataFrame) -> TheResult<DataFrame> {
  Ok(df)
}

/// Apply transformations specific to variants dataset
async fn apply_variants_transformations(df: DataFrame) -> TheResult<DataFrame> {
  Ok(df)
}

/// Print processing summary
fn print_summary(results: &[(String, bool)]) {
  println!("\n📋 Bronze Layer Processing Summary:");
  println!("==================================");

  let successful = results.iter().filter(|(_, success)| *success).count();
  let total = results.len();

  for (name, success) in results {
    let status = if *success { "✅" } else { "❌" };
    println!("{status} {name}");
  }

  println!(
    "\n📊 Results: {successful}/{total} datasets processed successfully"
  );

  if successful == total {
    println!("🎉 All datasets processed successfully!");
  } else {
    println!("⚠️  Some datasets failed to process. Check logs above.");
  }
}

/// Utility function for batch processing with custom options
pub async fn process_datasets_with_options(
  dataset_names: &[&str],
  chunk_size: Option<usize>,
  low_memory: bool,
) -> TheResult<()> {
  println!("🏗️  Starting Custom Bronze Layer Processing...");

  for &name in dataset_names {
    let config = Config::new(name)?
      .with_chunk_size(chunk_size)
      .with_low_memory_mode(low_memory);

    match process_dataset(&config).await {
      Ok(_) => println!("✅ Successfully processed: {name}"),
      Err(e) => eprintln!("❌ Failed to process {name}: {e}"),
    }
  }

  Ok(())
}

/// Get information about all datasets without processing
pub async fn inspect_datasets() -> TheResult<()> {
  println!("🔍 Dataset Inspection:");
  println!("======================");

  let dataset_names = [
    "credits", "crew", "profiles", "ratings", "series", "title", "variants",
  ];

  for name in dataset_names {
    match Config::new(name) {
      Ok(config) => {
        let exists = config.source_exists();
        let size = if exists {
          config.file_size().unwrap_or(0) / 1_024 / 1_024
        } else {
          0
        };

        let status = if exists { "✅" } else { "❌" };
        println!(
          "{} {} - {} MB - {}",
          status,
          name,
          size,
          config.path_as_str().unwrap_or("Invalid path")
        );
      }
      Err(e) => {
        println!("❌ {name} - Error: {e}");
      }
    }
  }

  Ok(())
}
