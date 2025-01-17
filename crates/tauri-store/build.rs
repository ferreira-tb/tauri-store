#[cfg(windows)]
use std::env::{current_dir, var};

fn main() {
  #[cfg(windows)]
  if is_workspace() {
    let manifest = current_dir()
      .unwrap()
      .join("assets")
      .join("manifest.xml");

    println!("cargo:rustc-link-arg=/WX");
    println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
    println!(
      "cargo:rustc-link-arg=/MANIFESTINPUT:{}",
      manifest.to_str().unwrap()
    );
  }
}

#[cfg(windows)]
fn is_workspace() -> bool {
  var("TAURI_STORE_WORKSPACE").is_ok_and(|it| it == "true")
}
