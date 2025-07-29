// -- Data Engineering Errors (types/data.rs) -- //

use super::{Category, Severity};
use std::fmt::{Display, Formatter, Result};

impl Display for Category {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Schema => write!(f, "Schema"),
      Self::Column => write!(f, "Column"),
      Self::DataFrame => write!(f, "DataFrame"),
      Self::DataType => write!(f, "DataType"),
      Self::Parquet => write!(f, "Parquet"),
      Self::Pipeline => write!(f, "Pipeline"),
      Self::DataQuality => write!(f, "DataQuality"),
      Self::LazyFrame => write!(f, "LazyFrame"),
      Self::Performance => write!(f, "Performance"),
      Self::Configuration => write!(f, "Configuration"),
      Self::System => write!(f, "System"),
    }
  }
}

impl Display for Severity {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Low => write!(f, "Low"),
      Self::Medium => write!(f, "Medium"),
      Self::High => write!(f, "High"),
      Self::Critical => write!(f, "Critical"),
    }
  }
}
