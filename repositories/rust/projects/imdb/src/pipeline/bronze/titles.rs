use crate::utilities::ingest;

use super::*;
const DATASET: &str = "Titles";
pub async fn execute(ctx: &SessionContext, cfg: &Config) -> Result<()> {
  println!("->> Begin >>-");
  let frame = ingest::source_to_frame(ctx, cfg, DATASET).await?;

  let mut df = frame.clone();

  df = error_msg!(
    df.sort(vec![col("tconst").sort(false, true)]),
    format!("Sorting failed {DATASET}")
  )?;
  // .into_diagnostic()
  // .wrap_err(format!("Sorting failed {DATASET}"))?;

  enriched_error!(
    df.show().await,
    format!("Dataframe analysis failure on: '{DATASET}'"),
    code = "df.show()" // help = "Ensure the file exists and you have read permissions. Check the file path is correct.",
                       // severity = Severity::Error
  )?;

  println!("-<< End <<-");
  Ok(())
}

// async fn ingest(
//   ctx: &SessionContext,
// ) -> Result<(), Box<dyn std::error::Error>> {
//   println!("Converting IMDB title.basics.tsv to Delta Lake...");

//   // Define schema for title.basics.tsv
//   let schema = Arc::new(Schema::new(vec![
//     Field::new("tconst", DataType::Utf8, false),
//     Field::new("titleType", DataType::Utf8, true),
//     Field::new("primaryTitle", DataType::Utf8, true),
//     Field::new("originalTitle", DataType::Utf8, true),
//     Field::new("isAdult", DataType::Boolean, true),
//     Field::new("startYear", DataType::Int32, true),
//     Field::new("endYear", DataType::Int32, true),
//     Field::new("runtimeMinutes", DataType::Int32, true),
//     Field::new("genres", DataType::Utf8, true),
//   ]));

//   // Configure CSV format for TSV (tab-separated)
//   let csv_format = CsvFormat::default()
//     .with_delimiter(b'\t')
//     .with_has_header(true)
//     .with_schema(schema.clone());

//   // Read TSV file
//   let df = ctx
//     .read_csv(
//       "title.basics.tsv",
//       CsvOptions::new().delimiter(b'\t').has_header(true),
//     )
//     .await?;

//   // Process the data (handle nulls, convert types, etc.)
//   let processed_df = df
//     .filter(col("tconst").is_not_null())?
//     .with_column(
//       "isAdult",
//       when(col("isAdult").eq(lit("1")), lit(true))
//         .when(col("isAdult").eq(lit("0")), lit(false))
//         .otherwise(lit(None::<bool>))?,
//     )?
//     .with_column(
//       "startYear",
//       when(col("startYear").eq(lit("\\N")), lit(None::<i32>)).otherwise(
//         col("startYear").cast_to(&DataType::Int32, schema.as_ref())?,
//       )?,
//     )?
//     .with_column(
//       "endYear",
//       when(col("endYear").eq(lit("\\N")), lit(None::<i32>)).otherwise(
//         col("endYear").cast_to(&DataType::Int32, schema.as_ref())?,
//       )?,
//     )?
//     .with_column(
//       "runtimeMinutes",
//       when(col("runtimeMinutes").eq(lit("\\N")), lit(None::<i32>)).otherwise(
//         col("runtimeMinutes").cast_to(&DataType::Int32, schema.as_ref())?,
//       )?,
//     )?;

//   // Collect the data
//   let batches = processed_df.collect().await?;

//   // Create Delta table
//   let table_path = "./delta_tables/title_basics";
//   let mut table = DeltaTableBuilder::from_uri(table_path)
//     .with_columns(schema.fields().iter().cloned())
//     .build()?;

//   // Write to Delta Lake
//   let mut writer = RecordBatchWriter::for_table(&table)?;
//   for batch in batches {
//     writer.write(batch).await?;
//   }
//   writer.flush_and_commit(&mut table).await?;

//   println!(
//     "Successfully converted title.basics.tsv to Delta Lake at {}",
//     table_path
//   );
//   Ok(())
// }
