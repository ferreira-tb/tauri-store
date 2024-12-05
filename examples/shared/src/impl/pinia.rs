use crate::CounterStore;
use tauri::AppHandle;
use tauri_plugin_pinia::ManagerExt;

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn get_counter(app: AppHandle) -> Option<i32> {
  app
    .pinia()
    .get("counter-store", "counter")
    .and_then(|counter| serde_json::from_value(counter).ok())
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn get_counter(app: AppHandle) -> Option<i32> {
  app
    .pinia()
    .get("counter-store", "counter")
    .await
    .and_then(|counter| serde_json::from_value(counter).ok())
}

#[tauri::command]
pub(crate) async fn print_counter(app: AppHandle) {
  let counter = try_get_counter(app).await;
  println!("counter: {counter}");
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn try_get_counter(app: AppHandle) -> i32 {
  app
    .pinia()
    .try_get::<i32>("counter-store", "counter")
    .unwrap()
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn try_get_counter(app: AppHandle) -> i32 {
  app
    .pinia()
    .try_get::<i32>("counter-store", "counter")
    .await
    .unwrap()
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn try_store_state(app: AppHandle) -> CounterStore {
  app
    .pinia()
    .try_store_state::<CounterStore>("counter-store")
    .expect("store must exist")
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn try_store_state(app: AppHandle) -> CounterStore {
  app
    .pinia()
    .try_store_state::<CounterStore>("counter-store")
    .await
    .expect("store must exist")
}
