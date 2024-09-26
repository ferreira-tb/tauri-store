use crate::manager::ManagerExt;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow};
use tauri_store::{Result, StoreState};
#[cfg(feature = "unstable-async")]
use {std::time::Duration, tauri_store::FutureExt};

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn clear_autosave() -> Result<()> {
  tauri_store::missing_feature!("unstable-async")
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn clear_autosave<R: Runtime>(app: AppHandle<R>) {
  app.pinia().clear_autosave();
}

#[tauri::command]
pub(crate) async fn disable_sync<R: Runtime>(app: AppHandle<R>, id: String) {
  app.pinia().disable_sync(id);
}

#[tauri::command]
pub(crate) async fn enable_sync<R: Runtime>(app: AppHandle<R>, id: String) {
  app.pinia().enable_sync(id);
}

#[tauri::command]
pub(crate) async fn get_pinia_path<R: Runtime>(app: AppHandle<R>) -> PathBuf {
  app.pinia().path().to_path_buf()
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn get_store_ids<R: Runtime>(app: AppHandle<R>) -> Vec<String> {
  app.pinia().ids()
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn get_store_ids<R: Runtime>(app: AppHandle<R>) -> Vec<String> {
  app.pinia().ids().await
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn get_store_path<R: Runtime>(app: AppHandle<R>, id: String) -> Result<PathBuf> {
  app
    .pinia()
    .with_store(id, |store| Ok(store.path()))
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn get_store_path<R: Runtime>(app: AppHandle<R>, id: String) -> Result<PathBuf> {
  app
    .pinia()
    .with_store(id, |store| async { store.path() }.boxed_ok())
    .await
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn load<R: Runtime>(app: AppHandle<R>, id: String) -> Result<StoreState> {
  app
    .pinia()
    .with_store(id, |store| Ok(store.state()))
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn load<R: Runtime>(app: AppHandle<R>, id: String) -> Result<StoreState> {
  app
    .pinia()
    .with_store(id, |store| async { store.state() }.boxed_ok())
    .await
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn patch<R: Runtime>(
  window: WebviewWindow<R>,
  id: String,
  state: StoreState,
) -> Result<()> {
  let app = window.app_handle().clone();
  app.pinia().with_store(id, move |store| {
    store.patch_with_source(state, window.label())
  })
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn patch<R: Runtime>(
  window: WebviewWindow<R>,
  id: String,
  state: StoreState,
) -> Result<()> {
  let app = window.app_handle().clone();
  app
    .pinia()
    .with_store(id, move |store| {
      async move { store.patch_with_source(state, window.label()) }.boxed()
    })
    .await
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn save<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.pinia().save(id)
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn save<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.pinia().save(id).await
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn save_all<R: Runtime>(app: AppHandle<R>) -> Result<()> {
  app.pinia().save_all()
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn save_all<R: Runtime>(app: AppHandle<R>) -> Result<()> {
  app.pinia().save_all().await
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn save_some<R: Runtime>(app: AppHandle<R>, ids: Vec<String>) -> Result<()> {
  app.pinia().save_some(&ids)
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn save_some<R: Runtime>(app: AppHandle<R>, ids: Vec<String>) -> Result<()> {
  app.pinia().save_some(&ids).await
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn set_autosave(_interval: u32) -> Result<()> {
  tauri_store::missing_feature!("unstable-async")
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn set_autosave<R: Runtime>(app: AppHandle<R>, interval: u32) {
  let duration = Duration::from_millis(interval.into());
  app.pinia().set_autosave(duration);
}

#[cfg(not(feature = "unstable-async"))]
#[tauri::command]
pub(crate) async fn unload<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.pinia().unload_store(&id)
}

#[cfg(feature = "unstable-async")]
#[tauri::command]
pub(crate) async fn unload<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.pinia().unload_store(&id).await
}
