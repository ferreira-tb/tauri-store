#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_pinia::ManagerExt;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CounterStore {
  counter: i32,
}

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
    .invoke_handler(tauri::generate_handler![
      get_counter,
      print_counter,
      try_get_counter,
      try_store_state
    ])
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
async fn get_counter(app: AppHandle) -> Option<i32> {
  app
    .pinia()
    .get("store", "counter")
    .and_then(|counter| serde_json::from_value(counter).ok())
}

#[tauri::command]
async fn print_counter(app: AppHandle) {
  let counter = try_get_counter(app).await;
  println!("counter: {counter}");
}

#[tauri::command]
async fn try_get_counter(app: AppHandle) -> i32 {
  app
    .pinia()
    .try_get::<i32>("store", "counter")
    .unwrap()
}

#[tauri::command]
async fn try_store_state(app: AppHandle) -> CounterStore {
  app
    .pinia()
    .try_store_state::<CounterStore>("store")
    .expect("store must exist")
}
