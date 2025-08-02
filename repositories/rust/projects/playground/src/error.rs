pub use miette::{
  Context, Diagnostic, IntoDiagnostic, MietteHandlerOpts, NamedSource, Report,
  Result, SourceSpan, WrapErr, bail, ensure, miette, set_hook, set_panic_hook,
};
use thiserror::Error;
use utils::Scope;

#[derive(Error, Diagnostic, Debug)]
#[error("Dataset initialization failed for '{dataset_name}'")]
#[diagnostic(
  code(dataset::init_failed),
  help("Try checking if the source data files exist and are accessible")
)]
pub struct DatasetInitError {
  #[source]
  pub source: anyhow::Error,
  pub dataset_name: String,
}

pub trait DatasetErrorExt {
  fn with_dataset_context(self, dataset_name: &str) -> Report;
}

impl DatasetErrorExt for Report {
  fn with_dataset_context(self, dataset_name: &str) -> Report {
    self.wrap_err(format!(
      "Failed to initialize dataset '{dataset_name}'\n\n\
            This error occurred during the dataset initialization phase.\n\
            Dataset: {dataset_name}\n\
            Location: bronze layer processing\n\n\
            Common causes:\n\
            • Source data files are missing or corrupted\n\
            • Database connection failed\n\
            • Required columns are missing from source data\n\
            • Schema file is missing: ./schemas/{dataset_name}.json"
    ))
  }
}

pub type TheResult<T> = miette::Result<T>;

pub trait ScopeExt {
  fn time_async<T, F, Fut>(
    &mut self,
    name: &str,
    f: F,
  ) -> impl Future<Output = TheResult<T>>
  where
    F: FnOnce() -> Fut,
    Fut: Future<Output = TheResult<T>>;
}

impl ScopeExt for Scope {
  fn time_async<T, F, Fut>(
    &mut self,
    name: &str,
    f: F,
  ) -> impl Future<Output = TheResult<T>>
  where
    F: FnOnce() -> Fut,
    Fut: Future<Output = TheResult<T>>,
  {
    self.time_async_task(name, f)
  }
}
