use super::path::{Files, Home};
use crate::*;

#[derive(Debug, Clone)]
pub struct Dataset {
  pub name: String,
  pub description: String,
  pub url: String,
  pub files: Files,
}

impl Default for Dataset {
  fn default() -> Self {
    let home = Home::default();
    let source = home.to_pathbuf().join("source");
    let import = home.to_pathbuf().join("import");
    Self {
      name: String::new(),
      description: String::new(),
      url: String::new(),
      files: Files {
        source,
        import
      }
    }
  }
}

impl Dataset {
  pub fn without_parent() -> Self {
    let home = Home::default().without_parent().parent.to_path_buf();
    Self {
      source: home.clone().join("source"),
      import: home.clone().join("import"),
      ..Default::default()
    }
  }
}

#[derive(Debug)]
pub struct Datasets {
  pub profiles: Dataset,
  pub credits: Dataset,
  pub titles: Dataset,
  pub variants: Dataset,
  pub ratings: Dataset,
  pub series: Dataset,
  pub crews: Dataset
}

impl Default for Datasets {
  fn default() -> Self {
    let home = Default
    let source_home = Dataset::without_parent().source;
    let import_home = Dataset::default().import;
    let ext = "tsv";

    let profiles_name = "profiles";
    let profiles_desc =
      "Contains profile information for actors, directors, etc.";
    let profiles_url = "https://datasets.imdbws.com/name.basics.tsv.gz";
    let profiles_source = source_home
      .join(filename_from_url(profiles_url).expect("This should not failed"));
    let profiles_import = import_home.join(profiles_name).with_extension(ext);

    let credits_name = "credits";
    let credits_desc = "Contains credits (roles and characters) information.";
    let credits_url = "https://datasets.imdbws.com/title.principals.tsv.gz";
    let credits_source = source_home
      .join(filename_from_url(credits_url).expect("This should not failed"));
    let credits_import = import_home.join(credits_name).with_extension(ext);

    let titles_name = "titles";
    let titles_desc = "Contains title and release information.";
    let titles_url = "https://datasets.imdbws.com/title.basics.tsv.gz";
    let titles_source = source_home
      .join(filename_from_url(titles_url).expect("This should not failed"));
    let titles_import = import_home.join(titles_name).with_extension(ext);

    let variants_name = "variants";
    let variants_desc = "Contains alternative titles and other information.";
    let variants_url = "https://datasets.imdbws.com/title.akas.tsv.gz";
    let variants_source = source_home
      .join(filename_from_url(variants_url).expect("This should not failed"));
    let variants_import = import_home.join(variants_name).with_extension(ext);

    let ratings_name = "ratings";
    let ratings_desc = "Contains rating and votes information.";
    let ratings_url = "https://datasets.imdbws.com/title.ratings.tsv.gz";
    let ratings_source = source_home
      .join(filename_from_url(ratings_url).expect("This should not failed"));
    let ratings_import = import_home.join(ratings_name).with_extension(ext);

    let series_name = "series";
    let series_desc = "Contains series information.";
    let series_url = "https://datasets.imdbws.com/title.episode.tsv.gz";
    let series_source = source_home
      .join(filename_from_url(series_url).expect("This should not failed"));
    let series_import = import_home.join(series_name).with_extension(ext);

    let crews_name = "crews";
    let crews_desc = "Contains director and writer information.";
    let crews_url = "https://datasets.imdbws.com/title.crew.tsv.gz";
    let crews_source = source_home
      .join(filename_from_url(crews_url).expect("This should not failed"));
    let crews_import = import_home.join(crews_name).with_extension(ext);

    Self {
      profiles: Dataset {
        name: String::from(profiles_name),
        description: String::from(profiles_desc),
        url: String::from(profiles_url),
        source: profiles_source,
        import: profiles_import
      },
      credits: Dataset {
        name: String::from(credits_name),
        description: String::from(credits_desc),
        url: String::from(credits_url),
        source: credits_source,
        import: credits_import
      },
      titles: Dataset {
        name: String::from(titles_name),
        description: String::from(titles_desc),
        url: String::from(titles_url),
        source: titles_source,
        import: titles_import
      },
      variants: Dataset {
        name: String::from(variants_name),
        description: String::from(variants_desc),
        url: String::from(variants_url),
        source: variants_source,
        import: variants_import
      },
      ratings: Dataset {
        name: String::from(ratings_name),
        description: String::from(ratings_desc),
        url: String::from(ratings_url),
        source: ratings_source,
        import: ratings_import
      },
      series: Dataset {
        name: String::from(series_name),
        description: String::from(series_desc),
        url: String::from(series_url),
        source: series_source,
        import: series_import
      },
      crews: Dataset {
        name: String::from(crews_name),
        description: String::from(crews_desc),
        url: String::from(crews_url),
        source: crews_source,
        import: crews_import
      }
    }
  }
}

impl Datasets {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn all(&self) -> [&Dataset; 7] {
    [
      &self.profiles,
      &self.credits,
      &self.titles,
      &self.variants,
      &self.ratings,
      &self.series,
      &self.crews
    ]
  }

  // pub fn from_cwd(&mut self) -> Result<&mut Self> {
}

// impl Datasets {
//   /// Creates datasets with default configuration (data/imdb).
//   pub fn with_defaults() -> Result<Self> {
//     Self::new(&Namespace::default())
//       .with_context(|| "Failed to create datasets with default
// configuration")   }

//   /// Creates datasets with flat namespace (no subdirectories).
//   pub fn flat() -> Result<Self> {
//     Self::new(&Namespace::flat())
//       .with_context(|| "Failed to create datasets with flat namespace")
//   }

//   /// Creates datasets with single namespace level.
//   pub fn with_namespace(namespace: impl Into<String>) -> Result<Self> {
//     Self::new(&Namespace::single(namespace))
//       .with_context(|| "Failed to create datasets with single namespace")
//   }

//   /// Creates datasets with custom nested namespace.
//   pub fn with_nested_namespace(
//     primary: impl Into<String>,
//     secondary: impl Into<String>
//   ) -> Result<Self> {
//     Self::new(&Namespace::nested(primary, secondary))
//       .with_context(|| "Failed to create datasets with nested namespace")
//   }
// }
