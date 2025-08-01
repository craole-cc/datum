use crate::*;

#[derive(Debug, Clone)]
pub struct Config {
  /// The name/key used to identify this dataset
  pub name: String,

  /// The path to the raw dataset file
  pub dataset: PathBuf,

  /// Directory containing the source files
  pub source_dir: PathBuf,

  /// Directory for imported/processed files
  pub import_dir: PathBuf,

  /// Extension of source files
  pub source_ext: String,

  /// Extension for imported files
  pub import_ext: String,

  /// Column separator (e.g. '\t' for TSV)
  pub delimiter: u8,

  /// Optional number of rows per chunk to read
  pub chunk_size: Option<usize>,

  /// Maximum number of chunks to read
  pub max_chunks: usize,

  /// Toggle low-memory parsing mode
  pub low_memory_mode: bool,

  /// Continue on parse errors
  pub ignore_errors: bool,

  /// Values to treat as null
  pub null_values: Vec<String>,

  /// Rows to skip before parsing (including or excluding header)
  pub skip_rows: usize,

  /// Processing mode
  pub mode: ProcessingMode,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      name: String::new(),
      dataset: PathBuf::new(),
      source_dir: PathBuf::new(),
      import_dir: PathBuf::new(),
      source_ext: "tsv".into(),
      import_ext: "parquet".into(),
      delimiter: b'\t',
      chunk_size: None,
      max_chunks: 1_000_000,
      low_memory_mode: false,
      ignore_errors: true,
      null_values: vec![
        "\\N".into(),
        "".into(),
        "N/A".into(),
        "N/A.".into(),
        "n/a".into(),
      ],
      skip_rows: 0,
      mode: ProcessingMode::default(),
    }
  }
}

impl Config {
  /// Create a new Config for the given dataset name
  pub fn new(name: &str) -> TheResult<Self> {
    // Get the actual filename from the mapping
    let dataset_filename = get_dataset_mapping(name)
      .context(format!("Unable to resolve mapping for {name}"))?;

    // Get the full path to the dataset file
    let dataset = get_dataset(dataset_filename)?;

    let source_dir = dataset
      .parent()
      .context("Unable to resolve parent dir")?
      .to_path_buf();

    let import_dir = source_dir
      .parent()
      .context("Unable to resolve parent dir")?
      .join("import");

    let ext = dataset
      .extension()
      .and_then(OsStr::to_str)
      .unwrap_or_default()
      .to_lowercase();

    let delimiter = match ext.as_str() {
      "tsv" => b'\t',
      "csv" => b',',
      _ => b',', // default fallback
    };

    Ok(Config {
      name: name.to_string(),
      dataset,
      source_dir,
      import_dir,
      source_ext: ext,
      delimiter,
      ..Default::default()
    })
  }

  /// Get the dataset path as a string reference
  pub fn path_as_str(&self) -> TheResult<&str> {
    self
      .dataset
      .to_str()
      .context("Failed to validate dataset path as UTF-8")
  }

  /// Get the source extension as a string reference
  pub fn ext_as_str(&self) -> &str {
    &self.source_ext
  }

  /// Get the import file path (dataset name with import extension in import_dir)
  pub fn import_path(&self) -> PathBuf {
    self
      .import_dir
      .join(format!("{}.{}", self.name, self.import_ext))
  }

  /// Get the import path as a string
  pub fn import_path_as_str(&self) -> TheResult<String> {
    Ok(
      self
        .import_path()
        .to_str()
        .context("Failed to convert import path to UTF-8")?
        .to_string(),
    )
  }

  /// Check if the source dataset file exists
  pub fn source_exists(&self) -> bool {
    self.dataset.exists() && self.dataset.is_file()
  }

  /// Check if the import directory exists, create it if it doesn't
  pub fn ensure_import_dir(&self) -> TheResult<()> {
    if !self.import_dir.exists() {
      std::fs::create_dir_all(&self.import_dir)
        .context("Failed to create import directory")?;
    }
    Ok(())
  }

  /// Get file size in bytes
  pub fn file_size(&self) -> TheResult<u64> {
    let meta =
      metadata(&self.dataset).context("Failed to get file metadata")?;
    Ok(meta.len())
  }

  /// Set chunk size
  pub fn with_chunk_size(mut self, chunk_size: Option<usize>) -> Self {
    self.chunk_size = chunk_size;
    self
  }

  /// Set max chunks
  pub fn with_max_chunks(mut self, max_chunks: usize) -> Self {
    self.max_chunks = max_chunks;
    self
  }

  /// Enable/disable low memory mode
  pub fn with_low_memory_mode(mut self, enabled: bool) -> Self {
    self.low_memory_mode = enabled;
    self
  }

  /// Add custom null values
  pub fn with_null_values(mut self, null_values: Vec<String>) -> Self {
    self.null_values = null_values;
    self
  }

  /// Set number of rows to skip
  pub fn with_skip_rows(mut self, skip_rows: usize) -> Self {
    self.skip_rows = skip_rows;
    self
  }

  /// Set custom delimiter
  pub fn with_delimiter(mut self, delimiter: u8) -> Self {
    self.delimiter = delimiter;
    self
  }
}

/// Get the full path to a dataset file
pub fn get_dataset(name: &str) -> TheResult<PathBuf> {
  // Build the "~/Downloads/data/imdb/download" path
  let home = directories::UserDirs::new()
    .context("Failed to get user directories")?
    .download_dir()
    .context("Failed to find downloads directory")?
    .join("data")
    .join("imdb")
    .join("download");

  // Append the requested filename
  let file = home.join(format!("{name}.tsv"));

  // Try to read its metadata (this errors if it doesn't exist or isn't accessible)
  let meta = metadata(&file)
    .context(format!("Failed to stat dataset file: {file:?}"))?;

  // Ensure it's actually a file (not a directory, symlink, etc.)
  ensure!(
    meta.is_file(),
    "Dataset path is not a regular file: {:?}",
    file
  );

  Ok(file)
}

/// Map user-friendly names to actual dataset filenames
pub fn get_dataset_mapping(key: &str) -> Option<&'static str> {
  match key {
    "profiles" => Some("name.basics"),
    "variants" => Some("title.akas"),
    "title" => Some("title.basics"),
    "crew" => Some("title.crew"),
    "series" => Some("title.episode"),
    "credits" => Some("title.principals"),
    "ratings" => Some("title.ratings"),
    _ => None,
  }
}
