use crate::manager::ManagerExt;
use std::path::PathBuf;
use std::time::Duration;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow};
use tauri_store::{with_store, Result, SaveStrategy, StoreOptions, StoreState};

#[tauri::command]
pub(crate) async fn clear_autosave<R: Runtime>(app: AppHandle<R>) {
  app.PLUGIN_NAME().clear_autosave();
}

#[tauri::command]
pub(crate) async fn get_default_save_strategy<R>(app: AppHandle<R>) -> SaveStrategy
where
  R: Runtime,
{
  app.PLUGIN_NAME().default_save_strategy()
}

#[tauri::command]
pub(crate) async fn get_PLUGIN_NAME_path<R: Runtime>(app: AppHandle<R>) -> PathBuf {
  app.PLUGIN_NAME().path().to_path_buf()
}

#[tauri::command]
pub(crate) async fn get_store_ids<R: Runtime>(app: AppHandle<R>) -> Vec<String> {
  app.PLUGIN_NAME().ids()
}

#[tauri::command]
pub(crate) async fn get_store_path<R>(app: AppHandle<R>, id: String) -> Result<PathBuf>
where
  R: Runtime,
{
  with_store(&app, id, |store| store.path())
}

#[tauri::command]
pub(crate) async fn get_save_strategy<R>(app: AppHandle<R>, id: String) -> Result<SaveStrategy>
where
  R: Runtime,
{
  with_store(&app, id, |store| store.save_strategy())
}

#[tauri::command]
pub(crate) async fn get_store_state<R>(app: AppHandle<R>, id: String) -> Result<StoreState>
where
  R: Runtime,
{
  app.PLUGIN_NAME().state(id)
}

#[tauri::command]
pub(crate) async fn load<R: Runtime>(app: AppHandle<R>, id: String) -> Result<StoreState> {
  with_store(&app, id, |store| store.state().clone())
}

#[tauri::command]
pub(crate) async fn patch<R>(window: WebviewWindow<R>, id: String, state: StoreState) -> Result<()>
where
  R: Runtime,
{
  let app = window.app_handle();
  let label = window.label().to_owned();
  with_store(app, id, move |store| store.patch_with_source(state, label))?
}

#[tauri::command]
pub(crate) async fn save<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.PLUGIN_NAME().save(id)
}

#[tauri::command]
pub(crate) async fn save_all<R: Runtime>(app: AppHandle<R>) -> Result<()> {
  app.PLUGIN_NAME().save_all()
}

#[tauri::command]
pub(crate) async fn save_all_now<R: Runtime>(app: AppHandle<R>) -> Result<()> {
  app.PLUGIN_NAME().save_all_now()
}

#[tauri::command]
pub(crate) async fn save_now<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.PLUGIN_NAME().save_now(id)
}

#[tauri::command]
pub(crate) async fn save_some<R: Runtime>(app: AppHandle<R>, ids: Vec<String>) -> Result<()> {
  app.PLUGIN_NAME().save_some(&ids)
}

#[tauri::command]
pub(crate) async fn save_some_now<R: Runtime>(app: AppHandle<R>, ids: Vec<String>) -> Result<()> {
  app.PLUGIN_NAME().save_some_now(&ids)
}

#[tauri::command]
pub(crate) async fn set_autosave<R: Runtime>(app: AppHandle<R>, interval: u64) {
  app
    .PLUGIN_NAME()
    .set_autosave(Duration::from_millis(interval));
}

#[tauri::command]
pub(crate) async fn set_save_strategy<R>(
  app: AppHandle<R>,
  id: String,
  strategy: SaveStrategy,
) -> Result<()>
where
  R: Runtime,
{
  with_store(&app, id, |store| store.set_save_strategy(strategy))
}

#[tauri::command]
pub(crate) async fn set_store_options<R>(
  window: WebviewWindow<R>,
  id: String,
  options: StoreOptions,
) -> Result<()>
where
  R: Runtime,
{
  let app = window.app_handle();
  let label = window.label().to_owned();
  with_store(app, id, move |store| {
    store.set_options_with_source(options, label)
  })?
}

#[tauri::command]
pub(crate) async fn unload<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.PLUGIN_NAME().unload_store(&id)
}
