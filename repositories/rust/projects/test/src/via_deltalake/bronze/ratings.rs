use super::*;

pub async fn execute() -> TheResult<()> {
  tsv_to_parquet("title.ratings", "ratings").await?;
  tsv_to_parquet("title.crew", "crew").await?;
  Ok(())
}
