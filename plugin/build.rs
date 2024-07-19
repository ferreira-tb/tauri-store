const COMMANDS: &[&str] = &["load", "save", "save_all", "set"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();
}
