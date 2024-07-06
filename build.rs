const COMMANDS: &[&str] = &["entries", "load", "set"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();
}
