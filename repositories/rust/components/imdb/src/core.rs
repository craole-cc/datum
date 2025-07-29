use crate::{Dataset, Error, Files, Home, Result};
/// IMDb dataset definitions - authoritative metadata and download URLs
const DATASET_DEFINITIONS: &[(&str, &str, &str)] = &[
  (
    "profiles",
    "Information about people involved in film and TV productions, including actors, directors, writers, and other crew members; contains unique IDs, birth/death years, known titles, and primary professions.",
    "https://datasets.imdbws.com/name.basics.tsv.gz",
  ),
  (
    "credits",
    "Detailed principal cast and crew credits for titles, including which actors played which characters, and main roles of directors, writers, and other key contributors associated with each title.",
    "https://datasets.imdbws.com/title.principals.tsv.gz",
  ),
  (
    "titles",
    "Core information about titles such as movies, TV series, episodes, and shorts; includes title types, primary and original titles, start and end years, runtime, and genres.",
    "https://datasets.imdbws.com/title.basics.tsv.gz",
  ),
  (
    "variants",
    "Alternative and localized title names for each title in various languages and regions, including original languages, types of title variations, and associated region codes.",
    "https://datasets.imdbws.com/title.akas.tsv.gz",
  ),
  (
    "ratings",
    "User ratings and vote counts collected for each title, providing average rating scores and the total number of votes contributing to those scores.",
    "https://datasets.imdbws.com/title.ratings.tsv.gz",
  ),
  (
    "series",
    "Mapping data linking TV episode titles to their parent TV series episodes, specifying the TV series identifier, season number, and episode number.",
    "https://datasets.imdbws.com/title.episode.tsv.gz",
  ),
  (
    "crews",
    "Lists of director(s) and writer(s) credited for each title, supporting analysis of creative contributors behind movies and TV productions.",
    "https://datasets.imdbws.com/title.crew.tsv.gz",
  ),
];

/// Collection of all IMDB datasets
#[derive(Debug)]
pub struct Datasets {
  datasets: Vec<Dataset>,
  home: Home,
}

impl Datasets {
  /// Creates a new collection of IMDB datasets with default configuration
  pub fn new() -> Result<Self> {
    Self::with_home(&Home::default())
  }

  /// Creates datasets with a custom home directory
  pub fn with_home(home: &Home) -> Result<Self> {
    let datasets = DATASET_DEFINITIONS
      .iter()
      .map(|(name, description, url)| {
        Self::create_dataset(name, description, url, home)
      })
      .collect::<Result<Vec<_>>>()?;

    Ok(Self {
      datasets,
      home: home.clone(),
    })
  }

  /// Returns all datasets as a slice
  pub fn all(&self) -> &[Dataset] {
    &self.datasets
  }

  /// Returns all datasets as a mutable slice
  pub fn all_mut(&mut self) -> &mut [Dataset] {
    &mut self.datasets
  }

  /// Returns an iterator over all datasets
  pub fn iter(&self) -> impl Iterator<Item = &Dataset> {
    self.datasets.iter()
  }

  /// Returns a mutable iterator over all datasets
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Dataset> {
    self.datasets.iter_mut()
  }

  /// Gets a dataset by name
  pub fn get(&self, name: &str) -> Option<&Dataset> {
    self.datasets.iter().find(|dataset| dataset.name() == name)
  }

  /// Gets a mutable dataset by name
  pub fn get_mut(&mut self, name: &str) -> Option<&mut Dataset> {
    self
      .datasets
      .iter_mut()
      .find(|dataset| dataset.name() == name)
  }

  /// Lists all available dataset names
  pub fn names(&self) -> Vec<&str> {
    self.datasets.iter().map(|d| d.name()).collect()
  }

  /// Gets the number of datasets
  pub fn len(&self) -> usize {
    self.datasets.len()
  }

  /// Checks if the collection is empty
  pub fn is_empty(&self) -> bool {
    self.datasets.is_empty()
  }

  /// Gets the current home configuration
  pub fn home(&self) -> &Home {
    &self.home
  }

  /// Updates all dataset paths with a new home directory
  pub fn with_home_mut(&mut self, home: &Home) -> Result<()> {
    for dataset in &mut self.datasets {
      let download_filename = Self::filename_from_url(&dataset.url)?;
      dataset.files =
        Files::with_home_and_filename(home, &dataset.name, download_filename)?;
    }
    self.home = home.clone();
    Ok(())
  }

  /// Updates all dataset paths with a new home directory (consuming version)
  pub fn with_home_updated(mut self, home: &Home) -> Result<Self> {
    self.with_home_mut(home)?;
    Ok(self)
  }

  /// Updates the parent directory for all datasets
  pub fn with_parent_mut(
    &mut self,
    parent: impl Into<std::path::PathBuf>,
  ) -> Result<()> {
    let new_home = self.home.clone().with_parent(parent);
    self.with_home_mut(&new_home)
  }

  /// Updates the parent directory for all datasets (consuming version)
  pub fn with_parent(
    mut self,
    parent: impl Into<std::path::PathBuf>,
  ) -> Result<Self> {
    self.with_parent_mut(parent)?;
    Ok(self)
  }

  /// Updates the base directory for all datasets
  pub fn with_base_mut(&mut self, base: impl Into<String>) -> Result<()> {
    let new_home = self.home.clone().with_base(base);
    self.with_home_mut(&new_home)
  }

  /// Updates the base directory for all datasets (consuming version)
  pub fn with_base(mut self, base: impl Into<String>) -> Result<Self> {
    self.with_base_mut(base)?;
    Ok(self)
  }

  /// Updates the category directory for all datasets
  pub fn with_category_mut(
    &mut self,
    category: impl Into<String>,
  ) -> Result<()> {
    let new_home = self.home.clone().with_category(category);
    self.with_home_mut(&new_home)
  }

  /// Updates the category directory for all datasets (consuming version)
  pub fn with_category(mut self, category: impl Into<String>) -> Result<Self> {
    self.with_category_mut(category)?;
    Ok(self)
  }

  /// Updates description for a specific dataset
  pub fn update_description(
    &mut self,
    dataset_name: &str,
    description: impl Into<String>,
  ) -> Result<()> {
    if let Some(dataset) = self.get_mut(dataset_name) {
      dataset.description = description.into();
      Ok(())
    } else {
      Err(Error::DatasetNotFound(dataset_name.to_string()))
    }
  }

  /// Updates multiple descriptions at once
  pub fn update_descriptions(
    &mut self,
    updates: &[(&str, &str)],
  ) -> Result<()> {
    for (name, description) in updates {
      self.update_description(name, *description)?;
    }
    Ok(())
  }

  /// Creates all necessary directories for all datasets
  pub fn create_all_dirs(&self) -> Result<()> {
    for dataset in &self.datasets {
      dataset.files.create_dirs()?;
    }
    Ok(())
  }

  /// Validates that all URLs are accessible/well-formed
  pub fn validate_urls(&self) -> Vec<(&str, Error)> {
    self
      .datasets
      .iter()
      .filter_map(|dataset| {
        if let Err(e) = Self::filename_from_url(&dataset.url) {
          Some((dataset.name.as_str(), e))
        } else {
          None
        }
      })
      .collect()
  }

  // Convenience getters that maintain the original API
  pub fn profiles(&self) -> Option<&Dataset> {
    self.get("profiles")
  }
  pub fn credits(&self) -> Option<&Dataset> {
    self.get("credits")
  }
  pub fn titles(&self) -> Option<&Dataset> {
    self.get("titles")
  }
  pub fn variants(&self) -> Option<&Dataset> {
    self.get("variants")
  }
  pub fn ratings(&self) -> Option<&Dataset> {
    self.get("ratings")
  }
  pub fn series(&self) -> Option<&Dataset> {
    self.get("series")
  }
  pub fn crews(&self) -> Option<&Dataset> {
    self.get("crews")
  }

  pub fn profiles_mut(&mut self) -> Option<&mut Dataset> {
    self.get_mut("profiles")
  }
  pub fn credits_mut(&mut self) -> Option<&mut Dataset> {
    self.get_mut("credits")
  }
  pub fn titles_mut(&mut self) -> Option<&mut Dataset> {
    self.get_mut("titles")
  }
  pub fn variants_mut(&mut self) -> Option<&mut Dataset> {
    self.get_mut("variants")
  }
  pub fn ratings_mut(&mut self) -> Option<&mut Dataset> {
    self.get_mut("ratings")
  }
  pub fn series_mut(&mut self) -> Option<&mut Dataset> {
    self.get_mut("series")
  }
  pub fn crews_mut(&mut self) -> Option<&mut Dataset> {
    self.get_mut("crews")
  }

  /// Helper function to create a dataset from metadata
  fn create_dataset(
    name: &str,
    description: &str,
    url: &str,
    home: &Home,
  ) -> Result<Dataset> {
    let download_filename = Self::filename_from_url(url)?;
    let files = Files::with_home_and_filename(home, name, download_filename)?;

    Ok(Dataset::new(name, description, url, files))
  }

  /// Extracts filename from URL - simple implementation for IMDB URLs
  pub fn filename_from_url(url: &str) -> Result<&str> {
    url
      .rsplit('/')
      .next()
      .filter(|name| !name.is_empty())
      .ok_or_else(|| {
        Error::InvalidUrl(format!("Failed to extract filename from URL: {url}"))
      })
  }
}

impl Default for Datasets {
  fn default() -> Self {
    Self::new().expect("Failed to create default datasets")
  }
}
