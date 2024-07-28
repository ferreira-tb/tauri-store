use std::fs;

const COMMANDS: &[&str] = &["load", "patch", "save", "save_all"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();

  fs::copy("../README.md", "../packages/guest-js/README.md").unwrap();

  /*let readme = fs::read_to_string("../README.md").unwrap();
  let lines = readme
    .lines()
    .map(|line| format!("//! {line}"))
    .collect::<Vec<_>>()
    .join("\n");

  let src = std::fs::read_to_string("src/lib.rs").unwrap();
  std::fs::write("src/lib.rs", format!("{lines}\n{src}")).unwrap();*/
}
