const COMMANDS: &[&str] = &[
  "clear_autosave",
  "get_default_save_strategy",
  "get_pinia_path",
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
  "set_save_strategy",
  "set_store_options",
  "unload",
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();
}
