use crate::CounterStore;
use tauri::AppHandle;
use tauri_plugin_PLUGIN_NAME::ManagerExt;

#[tauri::command]
pub(crate) async fn get_counter(app: AppHandle) -> Option<i32> {
  app
    .PLUGIN_NAME()
    .get("counter-store", "counter")
    .and_then(|counter| serde_json::from_value(counter).ok())
}

#[tauri::command]
pub(crate) async fn print_counter(app: AppHandle) {
  let counter = try_get_counter(app).await;
  println!("counter: {counter}");
}

#[tauri::command]
pub(crate) async fn try_get_counter(app: AppHandle) -> i32 {
  app
    .PLUGIN_NAME()
    .try_get::<i32>("counter-store", "counter")
    .unwrap()
}

#[tauri::command]
pub(crate) async fn try_store_state(app: AppHandle) -> CounterStore {
  app
    .PLUGIN_NAME()
    .try_store_state::<CounterStore>("counter-store")
    .expect("store must exist")
}
