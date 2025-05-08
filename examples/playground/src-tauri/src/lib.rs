use std::time::Duration;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_pinia::{BoxResult, SaveStrategy};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .plugin(
      tauri_plugin_pinia::Builder::new()
        .autosave(Duration::from_secs(60))
        .default_save_strategy(SaveStrategy::throttle_secs(3))
        .pretty(true)
        .save_denylist(["dont-save-1", "dont-save-2"])
        .sync_denylist(["dont-sync-1", "dont-sync-2"])
        .build(),
    )
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
