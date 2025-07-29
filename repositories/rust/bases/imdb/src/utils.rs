use crate::*;

pub fn filename_from_url(url: &str) -> Result<&str> {
  url
    .rsplit('/')
    .next()
    .filter(|name| !name.is_empty())
    .context("Failed to extract filename from URL: {url}")
}
// pub fn url_filename_ext(url: &str) -> Option<(&str, &str)> {
//   url
//     .rsplit('/')
//     .next()
//     .filter(|name| !name.is_empty())
//     .map(|filename| {
//       let mut parts = filename.splitn(2, '.');
//       let name = parts.next().unwrap_or("").to_string();
//       let ext = parts.next().unwrap_or("").to_string();
//       (name, ext)
//     })
// }
