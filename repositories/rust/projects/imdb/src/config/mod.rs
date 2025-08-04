use crate::*;

mod ingestion;

#[derive(Default, Debug)]
pub struct Config {
  pub datasets: imdb_dataset::Datasets,
  pub ingestion: ingestion::Config,
}

pub fn init() -> Result<()> {
  let imdb = &mut Config::default();
  // debug!("Initialized config {imdb:#?}");
  test()?;
  // ingestion::execute(imdb);
  Ok(())
}

pub fn test() -> Result<()> {
  let result = Path::new("/this/path/does/not/exist.csv");
  let path_result = if result.exists() {
    Ok(())
  } else {
    Err(std::io::Error::new(
      std::io::ErrorKind::NotFound,
      "Path does not exist",
    ))
  };
  // enriched_error!(path_result, "Something went wrong")?;
  trace_call!(detect_delimiter(result))?;
  // match trace_call!(result.exists()) {
  //   Ok(delimiter) => println!("Testing {delimiter:#?}"),
  //   Err(e) => eprintln!("{e:?}"),
  // }
  println!("->> Text Complete <<-");
  Ok(())
}
