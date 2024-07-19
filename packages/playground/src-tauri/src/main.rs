#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_pinia::init())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .setup(|app| {
      let handle = app.handle();
      (1..=4).for_each(|id| open_window(handle, id));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn open_window(app: &AppHandle, id: u8) {
  let label = format!("window-{id}");
  let url = format!("src/windows/{label}/index.html");
  WebviewWindowBuilder::new(app, &label, WebviewUrl::App(url.into()))
    .title(label)
    .inner_size(300.0, 300.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(true)
    .build()
    .unwrap();
}
