const COMMANDS: &[&str] = &["load", "patch", "save", "save_all", "unload"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();
}
