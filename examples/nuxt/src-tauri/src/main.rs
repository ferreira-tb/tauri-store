#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use example_shared::build;

fn main() {
  build()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
