use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Dataset {
  pub name: String,
  pub description: String,
  pub url: String,
  pub source: PathBuf,
  pub import: PathBuf,
}

impl Dataset {
  pub fn new<Name: AsRef<str>, Desc: AsRef<str>, Url: AsRef<str>>(
    name: Name,
    description: Desc,
    url: Url,
    no_data_namespace: bool,
    no_imdb_namespace: bool,
  ) -> Self {
    let downloads_home = directories::UserDirs::new()
      .and_then(|dirs| Some(dirs.download_dir()?.to_path_buf()))
      .or_else(|| Some(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))))
      .unwrap_or(PathBuf::from("."));

    let data_home = if no_data_namespace && no_imdb_namespace {
      downloads_home
    } else if no_imdb_namespace {
      downloads_home.join("data")
    } else if no_data_namespace {
      downloads_home.join("imdb")
    } else {
      downloads_home.join("data").join("imdb")
    };

    Self {
      name: name.as_ref().to_string(),
      description: description.as_ref().to_string(),
      url: url.as_ref().to_string(),
      source: data_home.join("source").join(
        url
          .as_ref()
          .split('/')
          .next_back()
          .expect("could not determine filename"),
      ),
      import: data_home
        .join("import")
        .join(format!("{}.tsv", name.as_ref())),
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
  pub crews: Dataset,
}

impl Default for Datasets {
  fn default() -> Self {
    Self {
      profiles: Dataset::new(
        "profiles",
        "Contains profile information for actors, directors, etc.",
        "https://datasets.imdbws.com/name.basics.tsv.gz",
        false,
        false,
      ),
      credits: Dataset::new(
        "credits",
        "Contains credits (roles and characters) information.",
        "https://datasets.imdbws.com/title.principals.tsv.gz",
        false,
        false,
      ),
      titles: Dataset::new(
        "titles",
        "Contains title and release information.",
        "https://datasets.imdbws.com/title.basics.tsv.gz",
        false,
        false,
      ),
      variants: Dataset::new(
        "variants",
        "Contains alternative titles and other information.",
        "https://datasets.imdbws.com/title.akas.tsv.gz",
        false,
        false,
      ),
      ratings: Dataset::new(
        "ratings",
        "Contains rating and votes information.",
        "https://datasets.imdbws.com/title.ratings.tsv.gz",
        false,
        false,
      ),
      series: Dataset::new(
        "series",
        "Contains series information.",
        "https://datasets.imdbws.com/title.episode.tsv.gz",
        false,
        false,
      ),
      crews: Dataset::new(
        "crews",
        "Contains director and writer information.",
        "https://datasets.imdbws.com/title.crew.tsv.gz",
        false,
        false,
      ),
    }
  }
}

impl Datasets {
  pub fn new(no_data_namespace: bool, no_imdb_namespace: bool) -> Self {
    Self {
      profiles: Dataset::new(
        "profiles",
        "Contains profile information for actors, directors, etc.",
        "https://datasets.imdbws.com/name.basics.tsv.gz",
        no_data_namespace,
        no_imdb_namespace,
      ),
      credits: Dataset::new(
        "credits",
        "Contains credits (roles and characters) information.",
        "https://datasets.imdbws.com/title.principals.tsv.gz",
        no_data_namespace,
        no_imdb_namespace,
      ),
      titles: Dataset::new(
        "titles",
        "Contains title and release information.",
        "https://datasets.imdbws.com/title.basics.tsv.gz",
        no_data_namespace,
        no_imdb_namespace,
      ),
      variants: Dataset::new(
        "variants",
        "Contains alternative titles and other information.",
        "https://datasets.imdbws.com/title.akas.tsv.gz",
        no_data_namespace,
        no_imdb_namespace,
      ),
      ratings: Dataset::new(
        "ratings",
        "Contains rating and votes information.",
        "https://datasets.imdbws.com/title.ratings.tsv.gz",
        no_data_namespace,
        no_imdb_namespace,
      ),
      series: Dataset::new(
        "series",
        "Contains series information.",
        "https://datasets.imdbws.com/title.episode.tsv.gz",
        no_data_namespace,
        no_imdb_namespace,
      ),
      crews: Dataset::new(
        "crews",
        "Contains director and writer information.",
        "https://datasets.imdbws.com/title.crew.tsv.gz",
        no_data_namespace,
        no_imdb_namespace,
      ),
    }
  }

  pub fn all(&self) -> [&Dataset; 7] {
    [
      &self.profiles,
      &self.credits,
      &self.titles,
      &self.variants,
      &self.ratings,
      &self.series,
      &self.crews,
    ]
  }
}
