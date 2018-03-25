#[macro_use] extern crate quicli;
use quicli::prelude::*;

// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct DuArgs {
    // Add a positional argument that the user has to supply:
    /// The folder
    folder: String,
}

fn execute_du(folder: String) {
  use std::process::Command;

  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", "echo in theory this should work on Windows"])
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg(format!("du -d 0 {}", folder))
      .output()
      .expect("failed to execute process")
  };

  let du_output: Vec<u8> = output.stdout;
  println!("{}", String::from_utf8(du_output).unwrap())
}

main!(|args: DuArgs| {
  execute_du(args.folder);
});
