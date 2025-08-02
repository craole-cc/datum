use crate::*;

/// (Display name, Dataset filename)
pub const DATASETS: [(&str, &str); 7] = [
  ("Credits", "title.principals"),
  ("Crew", "title.crew"),
  ("Profiles", "name.basics"),
  ("Ratings", "title.ratings"),
  ("Series", "title.episode"),
  ("Title", "title.basics"),
  ("Variants", "title.akas"),
];

#[derive(Debug, Clone)]
pub struct Config {
  /// The name/key used to identify this dataset
  pub name: String,

  /// The path to the raw dataset file
  pub dataset: PathBuf,

  /// Directory containing the import files
  pub source_dir: PathBuf,
  /// Directory for imported/processed files
  pub import_dir: PathBuf,

  /// Exteimport of source files
  pub source_ext: String,

  /// Extension for imported files
  pub import_ext: String,

  /// Column separator (e.g. '\t' for TSV)
  pub delimiter: u8,

  /// of the u8imported dataset Optional number of rows per chunk to read
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
  pub parquet_compression: Option<String>,
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
        "N/A".into(),
        "N/A.".into(),
        "n/a".into(),
      ],
      skip_rows: 0,
      mode: ProcessingMode::default(),
      parquet_compression: Some("snappy".into()),
    }
  }
}

impl Config {
  /// Create a new Config for the given dataset name
  pub fn new(name: &str) -> TheResult<Self> {
    let name = name.trim().to_lowercase();
    // Get the actual filename from the mapping
    let dataset_filename = get_dataset_mapping(&name)
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

  /// Returns an error if the dataset path is missing or not a file.
  pub fn ensure_source_exists(&self) -> TheResult<()> {
    // 1. Probe existence, surfacing any I/O errors
    let exists = self
      .dataset
      .try_exists()
      .into_diagnostic()
      .wrap_err("Failed to check dataset path existence")?;
    ensure!(exists, "dataset not found at {}", self.dataset.display());

    // 2. Verify it’s a file (not a directory or symlink to dir)
    let metadata = self
      .dataset
      .metadata()
      .into_diagnostic()
      .wrap_err("Failed to retrieve dataset metadata")?;
    ensure!(
      metadata.is_file(),
      "dataset path is not a file: {}",
      self.dataset.display()
    );

    Ok(())
  }

  /// Returns an error if the dataset path is missing or not a file.
  pub fn ensure_import_exists(&self) -> TheResult<()> {
    // 1. Probe existence, surfacing any I/O errors
    let exists = self
      .import_path()
      .try_exists()
      .into_diagnostic()
      .wrap_err("Failed to check import path existence")?;
    // .context("failed to check import path existence")?;
    ensure!(
      exists,
      "import file not found at {}",
      self.dataset.display()
    );

    // 2. Verify it’s a file (not a directory or symlink to dir)
    let metadata = self
      .dataset
      .metadata()
      .into_diagnostic()
      .wrap_err("Failed to retrieve metadata of the imported dataset")?;
    // .context("failed to retrieve metadata of the imported dataset")?;
    ensure!(
      metadata.is_file(),
      "the imported dataset path is not a file: {}",
      self.dataset.display()
    );

    Ok(())
  }

  /// Check if the import directory exists, create it if it doesn't
  pub fn ensure_import_dir(&self) -> TheResult<()> {
    if !self.import_dir.exists() {
      create_dir_all(&self.import_dir)
        .into_diagnostic()
        .wrap_err("Failed to create import directory")?;
      // .context("Failed to create import directory")?;
    }
    Ok(())
  }

  /// Get file size in bytes
  pub fn file_size(&self) -> TheResult<u64> {
    let meta = metadata(&self.dataset)
      .into_diagnostic()
      .wrap_err("Failed to get file metadata")?;
    // .context("Failed to get file metadata")?;
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
    .into_diagnostic()
    .wrap_err(format!("Failed to stat dataset file: {file:?}"))?;
  // .context(format!("Failed to stat dataset file: {file:?}"))?;

  // Ensure it's actually a file (not a directory, symlink, etc.)
  ensure!(
    meta.is_file(),
    "Dataset path is not a regular file: {:?}",
    file
  );

  Ok(file)
}

/// Lookup dataset filename by display name (case-insensitive)
pub fn get_dataset_mapping(display_name: &str) -> Option<&'static str> {
  DATASETS
    .iter()
    .find(|(pretty, _)| pretty.eq_ignore_ascii_case(display_name))
    .map(|(_, filename)| *filename)
}
