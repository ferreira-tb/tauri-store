mod migration;

use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tauri_store::BoxResult;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .plugin(
      tauri_store::Builder::new()
        .on_before_each_migration(|_| ())
        .migrations("my-store", migration::all())
        .build_plugin(),
    )
    .setup(|app| open_window(app.handle()))
    .run(tauri::generate_context!())
    .unwrap();
}

fn open_window(app: &AppHandle) -> BoxResult<()> {
  let url = WebviewUrl::App("index.html".into());
  WebviewWindowBuilder::new(app, "main", url)
    .title("Migration")
    .inner_size(300.0, 500.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(true)
    .always_on_top(true)
    .build()?;

  Ok(())
}
