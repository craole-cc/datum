use crate::Datasets;

pub fn clean(datasets: &Datasets, force: bool) {
  println!("{datasets:#?} Cleaning");
  println!("Force: {force}");
  // TODO: Store the timestamp when cleaned so that cleaning can be skipped if the already done
}
