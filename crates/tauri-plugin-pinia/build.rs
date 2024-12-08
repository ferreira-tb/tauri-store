const COMMANDS: &[&str] = &[
  "clear_autosave",
  "get_pinia_path",
  "get_store_ids",
  "get_store_path",
  "get_store_state",
  "load",
  "patch",
  "save",
  "save_all",
  "save_some",
  "set_autosave",
  "unload",
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();
}
