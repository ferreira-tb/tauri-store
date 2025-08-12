use std::error::Error;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_pinia::init())
    .setup(|app| setup(app.handle()))
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .unwrap();
}

fn setup(app: &AppHandle) -> Result<(), Box<dyn Error>> {
  let url = WebviewUrl::App("index.html".into());
  WebviewWindowBuilder::new(app, "main", url).build()?;
  Ok(())
}
