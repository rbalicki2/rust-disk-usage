// #[macro_use] extern crate quicli;
// use quicli::prelude::*;

fn main() {
  use std::process::Command;

  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", "echo in theory this should work on Windows"])
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("du -d 0 .")
      .output()
      .expect("failed to execute process")
  };

  let du_output: Vec<u8> = output.stdout;
  println!("{}", String::from_utf8(du_output).unwrap())
}
