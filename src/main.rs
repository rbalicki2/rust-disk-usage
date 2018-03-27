#[macro_use] extern crate quicli;
use quicli::prelude::*;

// mostly equivalent to
// find ./src/ -type f -exec ls -l {} \; | awk '{sum += $5} END {print sum}'

// Add cool slogan for your app here, e.g.:
/// Get the size of all files in a folder
#[derive(Debug, StructOpt)]
struct DuArgs {
    // Add a positional argument that the user has to supply:
    /// The folder
    folder: String,
}

fn get_folder_size(folder: &str) -> u64 {
  use std::fs;
  let metadata = fs::metadata(folder).unwrap();

  if metadata.is_dir() {
    let paths = fs::read_dir(folder).unwrap();
    return paths
      .map(|p| {
        let p = p.unwrap();
        get_folder_size(p.path().to_str().unwrap())
      })
      .fold(0, |acc, size| acc + size)
  }
  metadata.len()
}

main!(|args: DuArgs| {
  let size = get_folder_size(&args.folder);
  println!("{}", size);
});
