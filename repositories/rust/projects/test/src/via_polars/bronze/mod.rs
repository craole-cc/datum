// src/via_polars/bronze/mod.rs
use super::*;
mod crew;
mod profiles;
mod ratings;
mod series;
mod titles;
mod variants;

pub async fn execute() -> TheResult<()> {
  ratings::execute().await?;
  profiles::execute().await?;
  titles::execute().await?;
  variants::execute().await?;
  series::execute().await?;
  crew::execute().await?;
  Ok(())
}

pub async fn init(config: Config) -> TheResult<()> {
  // Determine processing mode based on file size or config
  let mode = determine_processing_mode(&config).await?;
  println!("Using processing mode: {mode:?}");

  match mode {
    ProcessingMode::SingleFrame => {
      process_single_frame(config).await?;
    }
    ProcessingMode::ChunkedSequential => {
      process_chunked_sequential(config).await?;
    }
    ProcessingMode::ChunkedConcurrent => {
      process_chunked_concurrent(config).await?;
    }
    ProcessingMode::StreamingLowMemory => {
      process_streaming_low_memory(config).await?;
    }
  }

  Ok(())
}

// Move the blocking Polars operations to a separate thread
async fn get_frame(cfg: Config) -> TheResult<DataFrame> {
  let dataset_path = cfg.dataset.clone();

  // Spawn blocking task for CPU-intensive Polars operations
  let frame = task::spawn_blocking(move || -> TheResult<DataFrame> {
    Ok(
      LazyCsvReader::new(get_path(dataset_path))
        .with_infer_schema_length(Some(0)) // Treat everything as strings
        .with_truncate_ragged_lines(true)
        .with_quote_char(None) // Disable quote parsing for TSV
        .with_comment_prefix(None) // Disable comment handling
        .with_ignore_errors(cfg.ignore_errors)
        .with_low_memory(cfg.low_memory_mode)
        .with_separator(cfg.delimiter)
        .with_has_header(cfg.skip_rows == 0)
        .with_skip_rows(if cfg.skip_rows == 0 {
          0
        } else {
          cfg.skip_rows + 1
        }) // +1 to skip header on subsequent chunks
        .with_n_rows(cfg.chunk_size)
        .with_null_values(get_null(cfg.null_values))
        .finish()?
        ._with_eager(true)
        .collect()?,
    )
  })
  .await??; // Double ? to handle both JoinError and our TheResult

  Ok(frame)
}

// For processing multiple chunks asynchronously
async fn get_frames_chunked(cfg: Config) -> TheResult<Vec<DataFrame>> {
  let mut frames = Vec::new();
  let chunk_size = cfg.chunk_size.unwrap_or(100_000);

  for chunk_idx in 0..cfg.max_chunks {
    let mut chunk_config = cfg.clone();
    chunk_config.chunk_size = Some(chunk_size);
    chunk_config.skip_rows = chunk_idx * chunk_size;

    match get_frame(chunk_config).await {
      Ok(frame) => {
        if frame.height() == 0 {
          break; // No more data
        }
        frames.push(frame);
      }
      Err(e) => {
        if cfg.ignore_errors {
          eprintln!("Warning: Failed to read chunk {chunk_idx}: {e}");
          continue;
        } else {
          return Err(e);
        }
      }
    }
  }

  Ok(frames)
}

// Async function to process frames with potential I/O operations
async fn process_frame_async(frame: DataFrame) -> TheResult<DataFrame> {
  // Spawn blocking for CPU-intensive transformations
  task::spawn_blocking(move || -> TheResult<DataFrame> {
    // Example transformations - replace with your actual logic
    let processed = frame
      .lazy()
      .with_columns([
        // Add any transformations here
        col("*").fill_null(lit("UNKNOWN")),
      ])
      .collect()?;

    Ok(processed)
  })
  .await?
}

fn get_null(null_values: Vec<String>) -> Option<NullValues> {
  if null_values.is_empty() {
    None
  } else {
    Some(NullValues::AllColumns(
      null_values.into_iter().map(Into::into).collect(),
    ))
  }
}

fn get_path(path: PathBuf) -> PlPath {
  PlPath::Local(path.into())
}

/// Single frame processing - load entire file at once
async fn process_single_frame(config: Config) -> TheResult<()> {
  println!("Processing as single frame...");

  let frame = get_frame(config).await?;
  let processed_frame = process_frame_async(frame).await?;

  println!("Processed frame shape: {:?}", processed_frame.shape());
  println!("Sample data: {:#?}", processed_frame.head(Some(5)));

  Ok(())
}

/// Chunked sequential processing - process chunks one by one
async fn process_chunked_sequential(config: Config) -> TheResult<()> {
  println!("Processing chunks sequentially...");

  let chunk_size = config.chunk_size.unwrap_or(10_000);
  let mut total_rows = 0;

  for chunk_idx in 0..config.max_chunks {
    let mut chunk_config = config.clone();
    chunk_config.chunk_size = Some(chunk_size);
    chunk_config.skip_rows = chunk_idx * chunk_size;

    match get_frame(chunk_config).await {
      Ok(frame) => {
        if frame.height() == 0 {
          break; // No more data
        }

        let processed = process_frame_async(frame).await?;
        total_rows += processed.height();

        println!("Processed chunk {}: {} rows", chunk_idx, processed.height());

        // Here you could write to Delta Lake, Parquet, etc.
        // write_to_storage(processed).await?;
      }
      Err(e) => {
        if config.ignore_errors {
          eprintln!("Warning: Failed to read chunk {chunk_idx}: {e}");
          continue;
        } else {
          return Err(e);
        }
      }
    }
  }

  println!("Total rows processed: {total_rows}");
  Ok(())
}

/// Chunked concurrent processing - process multiple chunks in parallel
async fn process_chunked_concurrent(config: Config) -> TheResult<()> {
  println!("Processing chunks concurrently...");

  let frames = get_frames_chunked(config).await?;
  println!("Read {} chunks", frames.len());

  // Process chunks concurrently
  let processed_futures: Vec<_> =
    frames.into_iter().map(process_frame_async).collect();

  let processed_frames =
    futures::future::try_join_all(processed_futures).await?;
  println!("Processed {} chunks", processed_frames.len());

  // Optionally combine all chunks
  if !processed_frames.is_empty() {
    let combined = combine_frames(processed_frames).await?;
    println!("Combined frame shape: {:?}", combined.shape());
  }

  Ok(())
}

/// Streaming low-memory processing - minimal memory footprint
async fn process_streaming_low_memory(config: Config) -> TheResult<()> {
  println!("Processing with streaming low-memory mode...");

  let chunk_size = config.chunk_size.unwrap_or(5_000); // Smaller chunks
  let mut total_rows = 0;
  let mut chunk_idx = 0;

  loop {
    let mut chunk_config = config.clone();
    chunk_config.chunk_size = Some(chunk_size);
    chunk_config.skip_rows = chunk_idx * chunk_size;
    chunk_config.low_memory_mode = true;

    let frame = match get_frame(chunk_config).await {
      Ok(frame) if frame.height() == 0 => break, // No more data
      Ok(frame) => frame,
      Err(e) if config.ignore_errors => {
        eprintln!("Warning: Failed to read chunk {chunk_idx}: {e}");
        chunk_idx += 1;
        continue;
      }
      Err(e) => return Err(e),
    };

    // Process immediately and don't keep in memory
    let processed = process_frame_async(frame).await?;
    total_rows += processed.height();

    println!("Streamed chunk {}: {} rows", chunk_idx, processed.height());

    // Write immediately to storage to free memory
    // write_to_delta_lake(processed).await?;

    chunk_idx += 1;

    // Optional: yield control to prevent blocking
    tokio::task::yield_now().await;
  }

  println!("Total rows streamed: {total_rows}");
  Ok(())
}

/// Determine the best processing mode based on file size and config
async fn determine_processing_mode(
  config: &Config,
) -> TheResult<ProcessingMode> {
  // Get file size
  let metadata = tokio::fs::metadata(&config.dataset).await?;
  let file_size_mb = metadata.len() as f64 / (1024.0 * 1024.0);

  println!("File size: {file_size_mb:.2} MB");

  // Decision logic based on file size and config
  let mode = if config.low_memory_mode {
    ProcessingMode::StreamingLowMemory
  } else if file_size_mb < 100.0 {
    ProcessingMode::SingleFrame
  } else if file_size_mb < 500.0 {
    ProcessingMode::ChunkedSequential
  } else {
    ProcessingMode::ChunkedConcurrent
  };

  Ok(mode)
}

/// Helper function to combine multiple DataFrames
async fn combine_frames(frames: Vec<DataFrame>) -> TheResult<DataFrame> {
  task::spawn_blocking(move || -> TheResult<DataFrame> {
    if frames.is_empty() {
      return Err(anyhow::anyhow!("No frames to combine"));
    }

    let mut combined = frames[0].clone();
    for frame in &frames[1..] {
      combined.vstack_mut(frame)?;
    }
    Ok(combined)
  })
  .await?
}
