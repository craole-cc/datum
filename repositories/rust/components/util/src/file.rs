use crate::*;

/// Represents the analysis results for a delimited data file (CSV, TSV, etc.)
#[derive(Debug, Clone)]
pub struct FileAnalysis {
  /// String representation of the file path
  pub path_str: String,
  /// File extension (lowercase, without the dot)
  pub extension: String,
  /// Detected delimiter character as a byte
  pub delimiter: u8,
  /// Parsed column names from the header line
  pub column_names: Vec<String>,
}

impl Display for FileAnalysis {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let delim_char = std::char::from_u32(self.delimiter as u32).unwrap_or(' ');

    write!(
      f,
      "FileAnalysis {{ path: \"{}\", extension: \"{}\", delimiter: {} ('{}'), column_names: [",
      self.path_str, self.extension, self.delimiter, delim_char
    )?;

    for (i, name) in self.column_names.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      write!(f, "\"{name}\"")?;
    }

    write!(f, "] }}")
  }
}

impl FileAnalysis {
  /// Creates a new FileAnalysis by analyzing the specified file.
  ///
  /// This function performs a complete analysis of a delimited data file:
  /// 1. Validates the file path exists
  /// 2. Extracts the file extension
  /// 3. Reads the first line (header)
  /// 4. Detects the delimiter character
  /// 5. Parses column names from the header
  ///
  /// # Arguments
  ///
  /// * `path` - Path to the file to analyze
  ///
  /// # Returns
  ///
  /// * `Ok(FileAnalysis)` - Complete analysis of the file
  /// * `Err(miette::Error)` - If any step of the analysis fails
  ///
  /// # Examples
  ///
  /// ```rust
  /// use std::path::Path;
  /// # use miette::Result;
  /// # #[derive(Debug, Clone)]
  /// # pub struct FileAnalysis {
  /// #     pub path_str: String,
  /// #     pub extension: String,
  /// #     pub delimiter: u8,
  /// #     pub column_names: Vec<String>,
  /// # }
  /// # impl FileAnalysis {
  /// #     pub fn new(path: &Path) -> Result<Self> {
  /// #         Ok(FileAnalysis {
  /// #             path_str: "test.csv".to_string(),
  /// #             extension: "csv".to_string(),
  /// #             delimiter: b',',
  /// #             column_names: vec!["name".to_string(), "age".to_string()],
  /// #         })
  /// #     }
  /// # }
  ///
  /// let analysis = FileAnalysis::new(Path::new("data.csv"))?;
  /// println!("Found {} columns with delimiter '{}'",
  ///          analysis.column_names.len(),
  ///          analysis.delimiter as char);
  /// # Ok::<(), miette::Error>(())
  /// ```
  pub fn new(path: &Path) -> Result<Self> {
    trace!("Triggered new FileAnalysis for path: {:?}", path);

    // Step 1: Validate and convert path to string
    let path_str = path_to_string(path).map_err(|original_error| {
      let location = format!("{}:{}:{}", file!(), line!(), column!());
      miette!(
        code = format!("path_to_string(path: {:?})", path),
        help =
          "Ensure the file path is valid and the file exists on the filesystem",
        severity = Severity::Error,
        labels = vec![LabeledSpan::at(
          0..path.to_string_lossy().len(),
          "file path"
        )],
        "Failed to analyze file path (at {})\n\nCaused by: {}",
        location,
        original_error
      )
    })?;

    // Step 2: Extract file extension (this cannot fail)
    let extension = get_file_extension(path);
    trace!("Detected file extension: '{}'", extension);

    // Step 3: Read the first line for analysis
    let first_line = read_first_line(path).map_err(|original_error| {
      let location = format!("{}:{}:{}", file!(), line!(), column!());
      miette!(
        code = format!("read_first_line(path: {:?})", path),
        help = "Verify the file is readable, not empty, and contains valid text data",
        severity = Severity::Error,
        labels = vec![
          LabeledSpan::at(0..path_str.len(), "file path"),
          LabeledSpan::at(0..0, "header line")
        ],
        "Failed to read header line from file (at {})\n\nCaused by: {}",
        location,
        original_error
      )
    })?;

    trace!(
      "Read header line ({} chars): {:?}",
      first_line.len(),
      first_line.chars().take(100).collect::<String>()
    );

    // Step 4: Detect delimiter from the header line
    let delimiter = detect_delimiter(&first_line).map_err(|original_error| {
      let location = format!("{}:{}:{}", file!(), line!(), column!());
      warn!("detect_delimiter(header_line: {:#?})", &first_line);
      warn!("detect_delimiter(header_line: {:#?})", &first_line.len().min(100));
      miette!(
        code = format!("detect_delimiter(header_line: {:#?})", &first_line),
        help = "Ensure the file uses a supported delimiter (comma, tab, pipe, or semicolon) and has multiple columns",
        severity = Severity::Error,
        labels = vec![
          LabeledSpan::at(0..first_line.len().min(100), "analyzed header line"),
          LabeledSpan::at(0..0, "delimiter detection")
        ],
        "Failed to detect delimiter in file header (at {})\n\nCaused by: {}",
        location,
        original_error
      )
    })?;

    trace!(
      "Detected delimiter: '{}' (byte: {})",
      delimiter as char, delimiter
    );

    // Step 5: Parse column names using the detected delimiter
    let column_names = parse_column_names(&first_line, delimiter).map_err(|original_error| {
            let location = format!("{}:{}:{}", file!(), line!(), column!());
            error!("parse_column_names(delimiter: '{}')", delimiter as char);
            miette!(
                help = "Verify the header line is properly formatted with valid column names separated by the detected delimiter",
                severity = Severity::Error,
                labels = vec![
                    LabeledSpan::at(0..first_line.len().min(100), "header line"),
                    LabeledSpan::at(0..0, "column parsing")
                ],
                "Failed to parse column names from header line (at {})\n\nCaused by: {}",
                location,
                original_error
            )
        })?;

    let analysis = FileAnalysis {
      path_str,
      extension,
      delimiter,
      column_names,
    };

    trace!("{:?}", &analysis);

    Ok(analysis)
  }

  /// Returns the delimiter as a character for display purposes
  pub fn delimiter_char(&self) -> char {
    self.delimiter as char
  }

  /// Returns the number of columns detected in the file
  pub fn column_count(&self) -> usize {
    self.column_names.len()
  }

  /// Checks if the file appears to be a CSV based on extension and delimiter
  pub fn is_csv(&self) -> bool {
    self.extension == "csv" || self.delimiter == b','
  }

  /// Checks if the file appears to be a TSV based on extension and delimiter
  pub fn is_tsv(&self) -> bool {
    self.extension == "tsv" || self.delimiter == b'\t'
  }

  /// Returns a summary string describing the file format
  pub fn format_summary(&self) -> String {
    let format_type = match self.delimiter {
      b',' => "CSV (comma-separated)",
      b'\t' => "TSV (tab-separated)",
      b'|' => "pipe-separated",
      b';' => "semicolon-separated",
      _ => "custom delimiter",
    };

    format!(
      "{} with {} columns (extension: {})",
      format_type,
      self.column_names.len(),
      if self.extension.is_empty() {
        "none"
      } else {
        &self.extension
      }
    )
  }
}

/// Converts a file path to a string representation, validating that the file exists.
///
/// This function checks if the file exists before converting the path to a string.
/// It's more robust than simple path-to-string conversion because it validates
/// the file system state.
///
/// # Arguments
///
/// * `path` - The file path to validate and convert
///
/// # Returns
///
/// * `Ok(String)` - String representation of the path if file exists
/// * `Err(miette::Error)` - If the file doesn't exist or path is invalid
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// # use miette::Result;
/// # fn path_to_string(path: &Path) -> Result<String> { Ok("test.csv".to_string()) }
///
/// let path_str = path_to_string(Path::new("existing_file.csv"))?;
/// println!("File path: {}", path_str);
/// # Ok::<(), miette::Error>(())
/// ```
fn path_to_string(path: &Path) -> Result<String> {
  trace!("Validating file path: {:?}", path);

  // Check if file exists first using manual error handling
  let path_exists = metadata(path).map_err(|io_error| {
        let location = format!("{}:{}:{}", file!(), line!(), column!());
        miette!(
            code = format!("std::fs::metadata(path: {:?})", path),
            help = "Verify the file path is correct and you have permission to access it",
            severity = Severity::Error,
            "Failed to check if file exists (at {})\n\nCaused by: {}",
            location,
            io_error
        )
    }).is_ok();

  if !path_exists {
    let path_display = path.display().to_string();
    return Err(miette!(
      code = "FILE_NOT_FOUND",
      help = "Check that the file path is correct and the file exists. Verify you have read permissions for the file and its parent directories.",
      severity = Severity::Error,
      labels = vec![LabeledSpan::at(0..path_display.len(), "file path")],
      "File does not exist: '{}'",
      path_display
    ));
  }

  // Convert to string
  let path_str = path.to_string_lossy().to_string();
  trace!("File path validated successfully: '{}'", path_str);

  Ok(path_str)
}

/// Extracts the file extension from a path, returning it in lowercase.
///
/// This function safely extracts the file extension, handling cases where
/// no extension exists. The extension is returned without the leading dot
/// and converted to lowercase for consistent comparison.
///
/// # Arguments
///
/// * `path` - The file path to extract extension from
///
/// # Returns
///
/// * `String` - The file extension in lowercase, or empty string if no extension
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// # fn get_file_extension(path: &Path) -> String { "csv".to_string() }
///
/// assert_eq!(get_file_extension(Path::new("data.CSV")), "csv");
/// assert_eq!(get_file_extension(Path::new("data.tsv")), "tsv");
/// assert_eq!(get_file_extension(Path::new("no_extension")), "");
/// ```
fn get_file_extension(path: &Path) -> String {
  let extension = path
    .extension()
    .and_then(|ext| ext.to_str())
    .unwrap_or("")
    .to_lowercase();

  trace!("Extracted file extension: '{}'", extension);
  extension
}

fn read_first_line(path: &Path) -> Result<String> {
  trace!("Starting read_first_line for path: {:?}", path);

  // 1. Open file
  trace!("Attempting to open file: {:?}", path);
  let file = enriched_error!(
    File::open(path),
    "Failed to open file for analysis",
    code = format!("File::open(path: {:?})", path),
    help = "Ensure the file exists, you have read permissions, and the path is correct.",
    severity = Severity::Error
  )?;
  trace!("File opened successfully: {:?}", path);

  // 2. Wrap in BufReader and prepare buffer
  let mut reader = BufReader::new(file);
  let mut buffer = String::new();
  let line_label = [LabeledSpan::at(0..0, "first line")];
  trace!("BufReader initialized; ready to read first line");

  // 3. Read exactly one line, capturing byte count
  let bytes_read = enriched_error!(
    reader.read_line(&mut buffer),
    "Failed to read first line from the file",
    code = format!(
      "BufReader::read_line(&mut buffer) in read_file(path: {:?})",
      path
    ),
    help = "Verify the file is not locked and contains valid UTF-8 text. Try opening it manually in a text editor.",
    severity = Severity::Error,
    labels = &line_label
  )?;
  trace!("Read {} bytes into buffer: {:?}", bytes_read, buffer);

  // 4. Empty‐file check (EOF on first read == no bytes)
  if bytes_read == 0 {
    trace!("No bytes read → file is completely empty");
    return Err(miette!(
      code = "EMPTY_FILE",
      help = "Provide a file with at least one line of data to analyze.",
      severity = Severity::Error,
      labels = vec![LabeledSpan::at(0..0, "empty content")],
      "Cannot analyze file: file appears to be completely empty"
    ));
  }

  // 5. Trim off trailing newline/carriage-return
  let trimmed = buffer.trim_end_matches(&['\r', '\n'][..]);
  trace!("Trimmed buffer to remove newline chars: {:?}", trimmed);

  // 6. Blank‐line check (line exists but only whitespace)
  if trimmed.trim().is_empty() {
    trace!("First line is blank or whitespace only");
    return Err(miette!(
      code = "BLANK_FIRST_LINE",
      help = "First line is blank or contains only whitespace. Provide a valid data line.",
      severity = Severity::Warning,
      labels = vec![LabeledSpan::at(0..buffer.len(), "blank first line")],
      "Cannot analyze file: first line is blank"
    ));
  }

  trace!("First line read and validated successfully: {:?}", trimmed);
  Ok(trimmed.to_string())
}

/// Detects the most likely delimiter character used in a CSV/TSV file.
///
/// Analyzes the first line of data to determine which delimiter (comma, tab, pipe, or semicolon)
/// appears most frequently. Returns an error if no delimiters are found or if the detection
/// is ambiguous.
///
/// # Arguments
///
/// * `line` - The first line of the file to analyze for delimiters
///
/// # Returns
///
/// * `Ok(u8)` - The detected delimiter as a byte (b',', b'\t', b'|', or b';')
/// * `Err(miette::Error)` - If no delimiters found or detection is ambiguous
///
/// # Examples
///
/// ```rust
/// # use miette::Result;
/// # fn detect_delimiter(line: &str) -> Result<u8> { Ok(b',') }
///
/// let csv_line = "name,age,city";
/// let delimiter = detect_delimiter(csv_line)?;
/// assert_eq!(delimiter, b',');
///
/// let tsv_line = "name\tage\tcity";
/// let delimiter = detect_delimiter(tsv_line)?;
/// assert_eq!(delimiter, b'\t');
/// # Ok::<(), miette::Error>(())
/// ```
fn detect_delimiter(line: &str) -> Result<u8> {
  // Validate input
  if line.is_empty() {
    return Err(miette!(
      code = "EMPTY_LINE",
      help = "Provide a non-empty line to analyze for delimiters",
      severity = Severity::Error,
      labels = vec![LabeledSpan::at(0..0, "empty input")],
      "Cannot detect delimiter: input line is empty"
    ));
  }

  // Count potential delimiters in first line
  let delimiter_counts = [
    (',', line.matches(',').count()),
    ('\t', line.matches('\t').count()),
    ('|', line.matches('|').count()),
    (';', line.matches(';').count()),
  ];

  // Log what we found for debugging
  trace!(
    "Delimiter analysis for line (length={}): commas={}, tabs={}, pipes={}, semicolons={}",
    line.len(),
    delimiter_counts[0].1,
    delimiter_counts[1].1,
    delimiter_counts[2].1,
    delimiter_counts[3].1
  );

  // Find the delimiter with the highest count
  let max_count = delimiter_counts
    .iter()
    .map(|(_, count)| *count)
    .max()
    .unwrap_or(0);

  if max_count == 0 {
    trace!(
      "No delimiters found in line: {:?}",
      line.chars().take(50).collect::<String>()
    );
    return Err(miette!(
      code = "NO_DELIMITER_FOUND",
      help = "The file may not be a delimited format, or it might use an unsupported delimiter. Supported delimiters: comma (,), tab (\\t), pipe (|), semicolon (;)",
      severity = Severity::Warning,
      labels = vec![
        LabeledSpan::at(0..line.len().min(100), "analyzed content"),
        LabeledSpan::at(0..0, "no delimiters found")
      ],
      "Could not detect delimiter: no common delimiters found in first line"
    ));
  }

  // Check for ambiguous detection (multiple delimiters with the same max count)
  let max_delimiters: Vec<_> = delimiter_counts
    .iter()
    .filter(|(_, count)| *count == max_count)
    .collect();

  if max_delimiters.len() > 1 {
    let analysis = delimiter_counts
      .iter()
      .map(|(delim, count)| {
        let delim_name = match delim {
          ',' => "commas",
          '\t' => "tabs",
          '|' => "pipes",
          ';' => "semicolons",
          _ => "unknown",
        };
        format!("{delim_name}:{count}")
      })
      .collect::<Vec<_>>()
      .join(", ");

    trace!("Ambiguous delimiter detection: {}", analysis);
    return Err(miette!(
      code = "AMBIGUOUS_DELIMITER",
      help = "Multiple delimiters found with equal counts. Consider manually specifying the delimiter or examining more lines of the file.",
      severity = Severity::Warning,
      labels = vec![
        LabeledSpan::at(0..line.len().min(100), "analyzed line"),
        LabeledSpan::at(0..analysis.len().min(50), "delimiter analysis")
      ],
      "Ambiguous delimiter detection: {}",
      analysis
    ));
  }

  // Return the winning delimiter
  let (winning_delimiter, count) = max_delimiters[0];
  let delimiter_byte = *winning_delimiter as u8;

  let format_name = match winning_delimiter {
    ',' => "CSV format (comma-separated)",
    '\t' => "TSV format (tab-separated)",
    '|' => "pipe-separated format",
    ';' => "semicolon-separated format",
    _ => "unknown format",
  };

  trace!("Detected {} with {} occurrences", format_name, count);
  Ok(delimiter_byte)
}

/// Parses column names from a header line using the specified delimiter.
///
/// Handles basic CSV parsing including quoted fields and escaped quotes.
/// Automatically cleans column names by trimming whitespace and removing
/// surrounding quotes if present.
///
/// # Arguments
///
/// * `line` - The header line containing column names
/// * `delimiter` - The delimiter character as a byte (from `detect_delimiter`)
///
/// # Returns
///
/// * `Ok(Vec<String>)` - Vector of cleaned column names
/// * `Err(miette::Error)` - If parsing fails or invalid input is provided
///
/// # Examples
///
/// ```rust
/// # use miette::Result;
/// # fn parse_column_names(line: &str, delimiter: u8) -> Result<Vec<String>> {
/// #     Ok(vec!["name".to_string(), "age".to_string()])
/// # }
///
/// let header = "name,age,\"full address\",phone";
/// let columns = parse_column_names(header, b',')?;
/// assert_eq!(columns, vec!["name", "age", "full address", "phone"]);
///
/// let tsv_header = "id\tname\tdescription";
/// let columns = parse_column_names(tsv_header, b'\t')?;
/// assert_eq!(columns, vec!["id", "name", "description"]);
/// # Ok::<(), miette::Error>(())
/// ```
fn parse_column_names(line: &str, delimiter: u8) -> Result<Vec<String>> {
  // Validate inputs
  if line.is_empty() {
    trace!("Empty line provided, returning empty column list");
    return Ok(Vec::new());
  }

  // Validate delimiter is printable (except for tab)
  let delimiter_char = delimiter as char;
  if !delimiter_char.is_ascii()
    || (!delimiter_char.is_ascii_graphic() && delimiter != b'\t')
  {
    return Err(miette!(
      code = "INVALID_DELIMITER",
      help = "Use a valid ASCII delimiter character such as comma (,), tab (\\t), pipe (|), or semicolon (;)",
      severity = Severity::Error,
      labels = vec![LabeledSpan::at(0..1, "invalid delimiter")],
      "Invalid delimiter byte: {} (not a valid ASCII delimiter)",
      delimiter
    ));
  }

  let trimmed_line = line.trim();
  trace!(
    "Parsing column names from line (length={}): {:?}",
    trimmed_line.len(),
    trimmed_line.chars().take(100).collect::<String>()
  );

  // Enhanced CSV parsing with better error handling
  let mut columns = Vec::new();
  let mut current_field = String::new();
  let mut in_quotes = false;
  let mut chars = trimmed_line.chars().enumerate().peekable();
  let mut quote_start_pos = None;

  while let Some((pos, ch)) = chars.next() {
    match ch {
      '"' if !in_quotes => {
        in_quotes = true;
        quote_start_pos = Some(pos);
      }
      '"' if in_quotes => {
        // Check for escaped quote (double quote)
        if let Some((_, next_ch)) = chars.peek() {
          if *next_ch == '"' {
            current_field.push('"');
            chars.next(); // consume the second quote
            continue;
          }
        }
        in_quotes = false;
        quote_start_pos = None;
      }
      ch if ch == delimiter_char && !in_quotes => {
        columns.push(current_field.trim().to_string());
        current_field.clear();
      }
      ch => {
        current_field.push(ch);
      }
    }
  }

  // Check for unterminated quotes
  if in_quotes {
    let quote_pos = quote_start_pos.unwrap_or(0);
    return Err(miette!(
      code = "UNTERMINATED_QUOTE",
      help =
        "Ensure all quoted fields are properly closed with matching quotes",
      severity = Severity::Error,
      labels = vec![
        LabeledSpan::at(
          quote_pos..quote_pos + 1,
          "unterminated quote starts here"
        ),
        LabeledSpan::at(
          trimmed_line.len()..trimmed_line.len(),
          "line ends here"
        )
      ],
      "Unterminated quoted field in column header"
    ));
  }

  // Don't forget the last field
  columns.push(current_field.trim().to_string());

  // Clean up column names (remove quotes if they wrap the entire field)
  let cleaned_columns: Vec<String> = columns
    .into_iter()
    .enumerate()
    .map(|(idx, col)| {
      let cleaned =
        if col.starts_with('"') && col.ends_with('"') && col.len() > 1 {
          col[1..col.len() - 1].to_string()
        } else {
          col
        };

      // Validate column names aren't empty after cleaning
      if cleaned.trim().is_empty() {
        trace!("Warning: Empty column name found at position {}", idx);
      }

      cleaned
    })
    .collect();

  // Validate we have at least one column
  if cleaned_columns.is_empty() {
    return Err(miette!(
      code = "NO_COLUMNS_FOUND",
      help = "Ensure the header line contains at least one column name",
      severity = Severity::Error,
      labels = vec![LabeledSpan::at(
        0..trimmed_line.len(),
        "analyzed header line"
      )],
      "No column names found after parsing header line"
    ));
  }

  // Check for duplicate column names
  let mut seen_columns: HashMap<String, usize> = HashMap::new();
  let mut duplicates = Vec::new();

  for (idx, col) in cleaned_columns.iter().enumerate() {
    let col_lower = col.to_lowercase();
    if let Some(first_idx) = seen_columns.get(&col_lower) {
      duplicates.push((col.clone(), *first_idx, idx));
    } else {
      seen_columns.insert(col_lower, idx);
    }
  }

  if !duplicates.is_empty() {
    let duplicate_info = duplicates
      .iter()
      .map(|(name, first, second)| {
        format!("'{name}' (positions {first} and {second})")
      })
      .collect::<Vec<_>>()
      .join(", ");

    trace!("Duplicate column names detected: {}", duplicate_info);
    // Note: We don't return an error here as some CSV files legitimately have duplicate columns
    // but we log it as a warning
  }

  trace!(
    "Successfully parsed {} column names: {:?}",
    cleaned_columns.len(),
    cleaned_columns.iter().take(10).collect::<Vec<_>>() // Show first 10 for brevity
  );

  Ok(cleaned_columns)
}

/// Convenience function that combines delimiter detection and column parsing.
///
/// This function first detects the delimiter from the header line, then parses
/// the column names using that delimiter.
///
/// # Arguments
///
/// * `header_line` - The first line of the CSV/TSV file containing column headers
///
/// # Returns
///
/// * `Ok((Vec<String>, u8))` - Tuple of (column names, detected delimiter)
/// * `Err(miette::Error)` - If detection or parsing fails
///
/// # Examples
///
/// ```rust
/// # use miette::Result;
/// # fn analyze_csv_header(line: &str) -> Result<(Vec<String>, u8)> {
/// #     Ok((vec!["name".to_string()], b','))
/// # }
///
/// let header = "name,age,city";
/// let (columns, delimiter) = analyze_csv_header(header)?;
/// assert_eq!(delimiter, b',');
/// assert_eq!(columns, vec!["name", "age", "city"]);
/// # Ok::<(), miette::Error>(())
/// ```
fn analyze_csv_header(header_line: &str) -> Result<(Vec<String>, u8)> {
  trace!("Starting CSV header analysis");

  let delimiter = detect_delimiter(header_line)?;
  let columns = parse_column_names(header_line, delimiter)?;

  trace!(
    "CSV header analysis complete: {} columns with delimiter '{}'",
    columns.len(),
    delimiter as char
  );

  Ok((columns, delimiter))
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use tempfile::NamedTempFile;

  #[test]
  fn test_path_to_string_existing_file() {
    let temp_file = NamedTempFile::new().unwrap();
    let result = path_to_string(temp_file.path());
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
  }

  #[test]
  fn test_path_to_string_nonexistent_file() {
    let result = path_to_string(Path::new("/nonexistent/file.txt"));
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(format!("{error}").contains("does not exist"));
  }

  #[test]
  fn test_get_file_extension() {
    assert_eq!(get_file_extension(Path::new("data.CSV")), "csv");
    assert_eq!(get_file_extension(Path::new("data.tsv")), "tsv");
    assert_eq!(get_file_extension(Path::new("data.txt")), "txt");
    assert_eq!(get_file_extension(Path::new("no_extension")), "");
    assert_eq!(get_file_extension(Path::new("hidden/.file")), "");
  }

  #[test]
  fn test_file_analysis_new() {
    let mut temp_file = NamedTempFile::new().unwrap();
    std::io::Write::write_all(&mut temp_file, b"name,age,city\nJohn,25,NYC\n")
      .unwrap();

    // Note: This test uses the mock functions above
    // In real usage, you'd need your actual implementation functions
    let result = FileAnalysis::new(temp_file.path());
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert!(!analysis.path_str.is_empty());
    assert_eq!(analysis.delimiter, b',');
    assert_eq!(analysis.column_names.len(), 3);
  }

  #[test]
  fn test_file_analysis_methods() {
    let analysis = FileAnalysis {
      path_str: "test.csv".to_string(),
      extension: "csv".to_string(),
      delimiter: b',',
      column_names: vec!["name".to_string(), "age".to_string()],
    };

    assert_eq!(analysis.delimiter_char(), ',');
    assert_eq!(analysis.column_count(), 2);
    assert!(analysis.is_csv());
    assert!(!analysis.is_tsv());
    assert!(analysis.format_summary().contains("CSV"));
    assert!(analysis.format_summary().contains("2 columns"));
  }

  #[test]
  fn test_file_analysis_tsv() {
    let analysis = FileAnalysis {
      path_str: "test.tsv".to_string(),
      extension: "tsv".to_string(),
      delimiter: b'\t',
      column_names: vec![
        "id".to_string(),
        "name".to_string(),
        "value".to_string(),
      ],
    };

    assert_eq!(analysis.delimiter_char(), '\t');
    assert_eq!(analysis.column_count(), 3);
    assert!(!analysis.is_csv());
    assert!(analysis.is_tsv());
    assert!(analysis.format_summary().contains("TSV"));
  }
}
