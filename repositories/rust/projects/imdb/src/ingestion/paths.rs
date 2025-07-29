// -- Path Management Module (ingestion/paths.rs) -- //

use super::*;

pub struct Paths {
  pub raw_file: PathBuf,
  pub chunks_dir: PathBuf,
  pub consolidated_file: PathBuf,
}

impl Paths {
  pub fn new(dataset: &Dataset) -> Result<Self> {
    let chunks_dir = dataset
      .files
      .raw
      .parent()
      .or_no_parent(&dataset.files.raw)?
      .join(&dataset.name);

    let consolidated_file = dataset
      .files
      .import
      .parent()
      .or_no_parent(&dataset.files.import)?
      .join(format!("{}.parquet", dataset.name));
    // let chunks_dir = dataset
    //   .files
    //   .raw
    //   .parent()
    //   .ok_or_else(|| ValidationError::no_parent_directory(&dataset.files.raw))?
    //   .join(&dataset.name);

    // let chunks_dir = dataset
    //   .files
    //   .raw
    //   .parent()
    //   .ok_or_else(|| {
    //     Error::Context(format!(
    //       "Cannot determine chunks directory for dataset: {}",
    //       dataset.name
    //     ))
    //   })?
    //   .join(&dataset.name);

    // let consolidated_file = dataset
    //   .files
    //   .import
    //   .parent()
    //   .ok_or_else(|| {
    //     Error::Context(format!(
    //       "Unable to derive the consolidated file path for dataset: {}",
    //       dataset.name
    //     ))
    //   })?
    //   .join(format!("{}.parquet", dataset.name));
    // let consolidated_file =
    // import_dir.join(format!("{}.parquet", dataset.name));

    Ok(Self {
      raw_file: dataset.files.raw.clone(),
      chunks_dir,
      consolidated_file,
    })
  }

  // pub fn ensure_chunks_directory(&self) -> Result<()> {
  //   if !self.chunks_dir.exists() {
  //     create_dir_all(&self.chunks_dir).map_err(|e| Error::FileSystem {
  //       message: format!(
  //         "Failed to create chunks directory: {:?}",
  //         self.chunks_dir
  //       ),
  //       source: e,
  //     })?;
  //     debug!("Created chunks directory: {:?}", self.chunks_dir);
  //   }
  //   Ok(())
  // }

  pub fn ensure_chunks_directory(&self) -> Result<()> {
    if !self.chunks_dir.exists() {
      create_dir_all(&self.chunks_dir).map_err(|e| {
        e.dir_create(&self.chunks_dir, Some("creating chunks directory"))
      })?;
      debug!("Created chunks directory: {:?}", self.chunks_dir);
    }
    Ok(())
  }
  pub fn get_chunk_path(&self, chunk_number: usize) -> PathBuf {
    self
      .chunks_dir
      .join(format!("chunk_{chunk_number:06}.parquet"))
  }

  pub fn list_existing_chunks(&self) -> Result<Vec<PathBuf>> {
    if !self.chunks_dir.exists() {
      return Ok(Vec::new());
    }

    let mut chunks = Vec::new();

    // Read directory with proper error handling
    for entry in std::fs::read_dir(&self.chunks_dir)
      .map_err(|e| e.dir_read(&self.chunks_dir, Some("listing chunk files")))?
    {
      let entry = entry.map_err(|e| {
        e.dir_read(&self.chunks_dir, Some("reading directory entry"))
      })?;
      let path = entry.path();

      if path.extension().and_then(|s| s.to_str()) == Some("parquet") {
        if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
          if name.starts_with("chunk_") {
            chunks.push(path);
          }
        }
      }
    }

    chunks.sort();
    Ok(chunks)
  }

  pub fn clean_old_chunks(&self) -> Result<()> {
    if !self.chunks_dir.exists() {
      return Ok(());
    }

    let chunks = self.list_existing_chunks()?;
    for chunk_path in chunks {
      if let Err(e) = std::fs::remove_file(&chunk_path) {
        warn!("Failed to remove old chunk {:?}: {}", chunk_path, e);
      } else {
        debug!("Removed old chunk: {:?}", chunk_path);
      }
    }
    Ok(())
  }

  pub fn are_chunks_up_to_date(&self) -> Result<bool> {
    let chunks = self.list_existing_chunks()?;
    if chunks.is_empty() {
      return Ok(false);
    }

    // Get raw file metadata
    let raw_meta = std::fs::metadata(&self.raw_file).map_err(|e| {
      e.file_read(&self.raw_file, Some("checking raw file modification time"))
    })?;
    let raw_modified = raw_meta.modified().map_err(|e| {
      e.file_read(&self.raw_file, Some("getting raw file modification time"))
    })?;

    // Check each chunk's modification time
    for chunk_path in chunks {
      let chunk_meta = std::fs::metadata(&chunk_path).map_err(|e| {
        e.file_read(&chunk_path, Some("checking chunk modification time"))
      })?;
      let chunk_modified = chunk_meta.modified().map_err(|e| {
        e.file_read(&chunk_path, Some("getting chunk modification time"))
      })?;

      if chunk_modified < raw_modified {
        return Ok(false);
      }
    }

    Ok(true)
  }

  pub fn is_consolidated_up_to_date(&self) -> Result<bool> {
    if !self.consolidated_file.exists() {
      return Ok(false);
    }

    // Get consolidated file metadata
    let consolidated_meta = std::fs::metadata(&self.consolidated_file)
      .map_err(|e| {
        e.file_read(
          &self.consolidated_file,
          Some("checking consolidated file modification time"),
        )
      })?;
    let consolidated_modified = consolidated_meta.modified().map_err(|e| {
      e.file_read(
        &self.consolidated_file,
        Some("getting consolidated file modification time"),
      )
    })?;

    // Get raw file metadata
    let raw_meta = std::fs::metadata(&self.raw_file).map_err(|e| {
      e.file_read(&self.raw_file, Some("checking raw file modification time"))
    })?;
    let raw_modified = raw_meta.modified().map_err(|e| {
      e.file_read(&self.raw_file, Some("getting raw file modification time"))
    })?;

    Ok(consolidated_modified >= raw_modified)
  }

  pub fn get_compression_stats(&self) -> Result<CompressionStats> {
    // Get raw file size
    let raw_size = std::fs::metadata(&self.raw_file)
      .map_err(|e| e.file_read(&self.raw_file, Some("getting raw file size")))?
      .len();

    // Get chunks and calculate total size
    let chunks = self.list_existing_chunks()?;
    let mut chunks_total_size = 0u64;

    for chunk_path in &chunks {
      let chunk_size = std::fs::metadata(chunk_path)
        .map_err(|e| e.file_read(chunk_path, Some("getting chunk size")))?
        .len();
      chunks_total_size += chunk_size;
    }

    // Get consolidated file size if it exists
    let consolidated_size = if self.consolidated_file.exists() {
      Some(
        std::fs::metadata(&self.consolidated_file)
          .map_err(|e| {
            e.file_read(
              &self.consolidated_file,
              Some("getting consolidated file size"),
            )
          })?
          .len(),
      )
    } else {
      None
    };

    Ok(CompressionStats {
      raw_size,
      chunks_total_size,
      consolidated_size,
      chunk_count: chunks.len(),
    })
  }
}

#[derive(Debug)]
pub struct CompressionStats {
  pub raw_size: u64,
  pub chunks_total_size: u64,
  pub consolidated_size: Option<u64>,
  pub chunk_count: usize,
}

impl CompressionStats {
  pub fn chunks_compression_ratio(&self) -> f64 {
    if self.chunks_total_size == 0 {
      0.0
    } else {
      self.raw_size as f64 / self.chunks_total_size as f64
    }
  }

  pub fn consolidated_compression_ratio(&self) -> Option<f64> {
    self.consolidated_size.map(|size| {
      if size == 0 {
        0.0
      } else {
        self.raw_size as f64 / size as f64
      }
    })
  }

  pub fn log_stats(&self, dataset_name: &str) {
    let raw_mb = self.raw_size as f64 / 1_048_576.0;
    let chunks_mb = self.chunks_total_size as f64 / 1_048_576.0;

    info!(
      "{} chunks compression: {:.1}MB -> {:.1}MB ({:.1}x reduction) across {} files",
      dataset_name,
      raw_mb,
      chunks_mb,
      self.chunks_compression_ratio(),
      self.chunk_count
    );

    if let Some(consolidated_size) = self.consolidated_size {
      let consolidated_mb = consolidated_size as f64 / 1_048_576.0;
      info!(
        "{} consolidated: {:.1}MB -> {:.1}MB ({:.1}x reduction)",
        dataset_name,
        raw_mb,
        consolidated_mb,
        self.consolidated_compression_ratio().unwrap_or(0.0)
      );
    }
  }
}
