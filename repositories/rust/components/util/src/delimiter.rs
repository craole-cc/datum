use crate::*;
pub fn detect_delimiter(path: &Path) -> Result<u8> {
  let result = || -> Result<u8> {
    let file_labels = [LabeledSpan::at(0..path.to_string_lossy().len(), "")];
    let file = enriched_error!(
      File::open(path),
      "Failed to open file for delimiter detection",
      code = "std::fs::File::open",
      help = "Ensure the file exists and you have read permissions. Check the file path is correct.",
      severity = Severity::Error,
      labels = &file_labels
    )?;

    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    let read_labels = [LabeledSpan::at(0..0, "first line")];
    enriched_error!(
      reader.read_line(&mut first_line),
      "Failed to read first line for delimiter detection",
      code = "FILE_READ",
      help = "File may be empty, corrupted, or contain invalid UTF-8. Try opening the file in a text editor to verify its contents.",
      severity = Severity::Error,
      labels = &read_labels
    )?;

    // Check if file is empty
    if first_line.trim().is_empty() {
      return Err(miette!(
        code = "EMPTY_FILE",
        help =
          "Provide a file with at least one line of data to detect delimiters",
        severity = Severity::Warning,
        labels = vec![LabeledSpan::at(0..0, "empty content")],
        "Cannot detect delimiter: file appears to be empty"
      ));
    }

    // Count potential delimiters in first line
    let comma_count = first_line.matches(',').count();
    let tab_count = first_line.matches('\t').count();
    let pipe_count = first_line.matches('|').count();
    let semicolon_count = first_line.matches(';').count();

    // Log what we found for debugging
    debug!(
      "Delimiter analysis: commas={}, tabs={}, pipes={}, semicolons={}",
      comma_count, tab_count, pipe_count, semicolon_count
    );

    match (comma_count, tab_count, pipe_count, semicolon_count) {
      (c, t, p, s) if c > t && c > p && c > s => {
        info!("Detected CSV format (comma-separated)");
        Ok(b',')
      }
      (c, t, p, s) if t > c && t > p && t > s => {
        info!("Detected TSV format (tab-separated)");
        Ok(b'\t')
      }
      (c, t, p, s) if p > c && p > t && p > s => {
        info!("Detected pipe-separated format");
        Ok(b'|')
      }
      (c, t, p, s) if s > c && s > t && s > p => {
        info!("Detected semicolon-separated format");
        Ok(b';')
      }
      (0, 0, 0, 0) => {
        warn!("No delimiters found, defaulting to comma");
        Err(miette!(
          code = "NO_DELIMITER_FOUND",
          help = "The file may not be a delimited format, or it might use an unsupported delimiter. Supported delimiters: comma (,), tab (\\t), pipe (|), semicolon (;)",
          severity = Severity::Warning,
          labels = vec![
            LabeledSpan::at(
              0..first_line.len().saturating_sub(1),
              "analyzed content"
            ),
            LabeledSpan::at(0..0, "no delimiters found")
          ],
          "Could not detect delimiter: no common delimiters found in first line"
        ))
      }
      _ => {
        warn!("Ambiguous delimiter detection, defaulting to comma");
        let analysis = format!(
          "commas:{comma_count}, tabs:{tab_count}, pipes:{pipe_count}, semicolons:{semicolon_count}"
        );
        Err(miette!(
          code = "AMBIGUOUS_DELIMITER",
          help = "Multiple delimiters found with similar counts. Consider manually specifying the delimiter.",
          severity = Severity::Warning,
          labels = vec![
            LabeledSpan::at(
              0..first_line.len().saturating_sub(1),
              "analyzed line"
            ),
            LabeledSpan::at(0..analysis.len(), "delimiter counts")
          ],
          "Ambiguous delimiter detection: {}",
          analysis
        ))?;
        Ok(b',') // This won't be reached due to the ? above
      }
    }
  };

  trace_fn!("detect_delimiter", result())
}
