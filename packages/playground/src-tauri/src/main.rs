#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_pinia::ManagerExt;

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
    .invoke_handler(tauri::generate_handler![print_counter])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn open_window(app: &AppHandle, id: u8) {
  let label = format!("window-{id}");
  let url = WebviewUrl::App("src/index.html".into());
  WebviewWindowBuilder::new(app, &label, url)
    .title(label)
    .inner_size(300.0, 300.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(true)
    .build()
    .unwrap();
}

#[tauri::command]
async fn print_counter(app: AppHandle) {
  let _ = app.with_store("store", |store| {
    if let Some(counter) = store.get("counter") {
      println!("counter: {counter}");
    }

    Ok(())
  });
}
