use crate::Datasets;
use std::path::PathBuf;

pub fn check(datasets: &Datasets) {
  println!("{datasets:#?} Checking");


  // Check only for the csv files
}

// pub fn check_all(
//   datasets: &Datasets,
// ) -> anyhow::Result<()> {
//   for ds in datasets.all() {
//     let source = if let Some(ref base) = data_dir {
//       base.join(ds.source.file_name().unwrap())
//     } else {
//       ds.source.clone()
//     };
//     if !source.exists() {
//       println!("Dataset {} missing at {:?}", ds.name, source);
//       return Err(anyhow::anyhow!("Missing datasets"));
//     }
//   }
//   println!("All datasets are present.");
//   Ok(())
// }
