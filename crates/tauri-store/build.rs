#[cfg(feature = "plugin")]
const COMMANDS: &[&str] = &[
  "clear_autosave",
  "get_default_save_strategy",
  "get_store_collection_path",
  "get_save_strategy",
  "get_store_ids",
  "get_store_path",
  "get_store_state",
  "load",
  "patch",
  "save",
  "save_all",
  "save_all_now",
  "save_now",
  "save_some",
  "save_some_now",
  "set_autosave",
  "set_store_collection_path",
  "set_save_strategy",
  "set_store_options",
  "unload",
];

fn main() {
  #[cfg(windows)]
  if is_workspace() {
    let manifest = std::env::current_dir()
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

  #[cfg(feature = "plugin")]
  tauri_plugin::Builder::new(COMMANDS).build();
}

#[cfg(windows)]
fn is_workspace() -> bool {
  std::env::var("TAURI_STORE_WORKSPACE").is_ok_and(|it| it == "true")
}
