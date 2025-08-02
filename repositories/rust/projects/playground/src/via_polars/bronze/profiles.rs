// src/via_polars/bronze/ratings.rs

use super::*;

pub async fn execute() -> TheResult<()> {
  let config = Config::new("name.basics.tsv")?.with_mode_frame();

  let frame = get_frame(config).await?;
  let processed_frame = process_frame_async(frame).await?;

  println!("{processed_frame:#?}",);

  Ok(())
}
