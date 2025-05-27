use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_pinia::BoxResult;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .plugin(tauri_plugin_pinia::init())
    .plugin(tauri_plugin_svelte::init())
    .plugin(tauri_plugin_valtio::init())
    .plugin(tauri_plugin_vue::init())
    .plugin(tauri_plugin_zustand::init())
    .setup(|app| open_window(app.handle()))
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .unwrap();
}

fn open_window(app: &AppHandle) -> BoxResult<()> {
  let url = WebviewUrl::App("index.html".into());
  WebviewWindowBuilder::new(app, "main", url)
    .title("Playground")
    .inner_size(300.0, 500.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(true)
    .always_on_top(true)
    .build()?;

  Ok(())
}
