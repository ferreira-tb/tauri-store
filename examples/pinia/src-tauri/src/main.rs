#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_pinia::ManagerExt;

#[cfg(feature = "unstable-async")]
use tauri::async_runtime::block_on;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CounterStore {
  counter: i32,
}

fn main() {
  let pinia = tauri_plugin_pinia::Builder::new()
    .pretty(true)
    .build();

  tauri::Builder::default()
    .plugin(pinia)
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .setup(|app| {
      let handle = app.handle();
      (1..=4).for_each(|id| open_window(handle, id));

      #[cfg(not(feature = "unstable-async"))]
      watch_counter(handle);
      #[cfg(feature = "unstable-async")]
      block_on(watch_counter(handle));

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
  let url = WebviewUrl::App("index.html".into());
  WebviewWindowBuilder::new(app, &label, url)
    .title(label)
    .inner_size(300.0, 500.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(true)
    .always_on_top(true)
    .build()
    .unwrap();
}

#[cfg(not(feature = "unstable-async"))]
fn watch_counter(app: &AppHandle) {
  let _ = app.pinia().watch("counter-store", |handle| {
    handle
      .pinia()
      .try_get::<i32>("counter-store", "counter")
      .inspect(|counter| println!("counter: {counter}"))
      .map(drop)
  });
}

#[cfg(feature = "unstable-async")]
async fn watch_counter(app: &AppHandle) {
  let _ = app
    .pinia()
    .watch("counter-store", |handle| {
      Box::pin(async move {
        handle
          .pinia()
          .try_get::<i32>("counter-store", "counter")
          .await
          .inspect(|counter| println!("counter: {counter}"))
          .map(drop)
      })
    })
    .await;
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
async fn get_counter(app: AppHandle) -> Option<i32> {
  app
    .pinia()
    .get("counter-store", "counter")
    .and_then(|counter| serde_json::from_value(counter).ok())
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
async fn get_counter(app: AppHandle) -> Option<i32> {
  app
    .pinia()
    .get("counter-store", "counter")
    .await
    .and_then(|counter| serde_json::from_value(counter).ok())
}

#[tauri::command]
async fn print_counter(app: AppHandle) {
  let counter = try_get_counter(app).await;
  println!("counter: {counter}");
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
async fn try_get_counter(app: AppHandle) -> i32 {
  app
    .pinia()
    .try_get::<i32>("counter-store", "counter")
    .unwrap()
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
async fn try_get_counter(app: AppHandle) -> i32 {
  app
    .pinia()
    .try_get::<i32>("counter-store", "counter")
    .await
    .unwrap()
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
async fn try_store_state(app: AppHandle) -> CounterStore {
  app
    .pinia()
    .try_store_state::<CounterStore>("counter-store")
    .expect("store must exist")
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
async fn try_store_state(app: AppHandle) -> CounterStore {
  app
    .pinia()
    .try_store_state::<CounterStore>("counter-store")
    .await
    .expect("store must exist")
}
