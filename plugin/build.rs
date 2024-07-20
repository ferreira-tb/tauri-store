const COMMANDS: &[&str] = &["load", "patch", "save", "save_all"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();
}
