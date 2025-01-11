use std::env::{current_dir, var};

fn main() {
  if cfg!(windows) && is_workspace() {
    embed_manifest();
  }
}

/// Prevent `STATUS_ENTRYPOINT_NOT_FOUND` error in tests.
fn embed_manifest() {
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

fn is_workspace() -> bool {
  var("__TAURI_STORE_WORKSPACE__").is_ok_and(|v| v == "true")
}
