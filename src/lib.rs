use tauri::plugin::TauriPlugin;
use tauri::Runtime;

pub fn ini<R: Runtime>() -> TauriPlugin<R> {
  tauri::plugin::Builder::new("pinia").build()
}
