// This file was autogenerated and should not be edited manually.
// Check the `codegen` command in the `tauri-store-cli` crate.

use crate::manager::ManagerExt;
use crate::{with_store, Result, SaveStrategy, StoreOptions, StoreState};
use std::path::PathBuf;
use std::time::Duration;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow};

#[tauri::command]
pub(crate) async fn clear_autosave<R>(app: AppHandle<R>)
where
  R: Runtime,
{
  app.store_collection().clear_autosave();
}

#[tauri::command]
pub(crate) async fn get_default_save_strategy<R>(app: AppHandle<R>) -> SaveStrategy
where
  R: Runtime,
{
  app.store_collection().default_save_strategy()
}

#[tauri::command]
pub(crate) async fn get_store_collection_path<R>(app: AppHandle<R>) -> PathBuf
where
  R: Runtime,
{
  app.store_collection().path()
}

#[tauri::command]
pub(crate) async fn get_store_ids<R>(app: AppHandle<R>) -> Vec<String>
where
  R: Runtime,
{
  app.store_collection().ids()
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
  app.store_collection().state(id)
}

#[tauri::command]
pub(crate) async fn load<R>(app: AppHandle<R>, id: String) -> Result<StoreState>
where
  R: Runtime,
{
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
pub(crate) async fn save<R>(app: AppHandle<R>, id: String) -> Result<()>
where
  R: Runtime,
{
  app.store_collection().save(id)
}

#[tauri::command]
pub(crate) async fn save_all<R>(app: AppHandle<R>) -> Result<()>
where
  R: Runtime,
{
  app.store_collection().save_all()
}

#[tauri::command]
pub(crate) async fn save_all_now<R>(app: AppHandle<R>) -> Result<()>
where
  R: Runtime,
{
  app.store_collection().save_all_now()
}

#[tauri::command]
pub(crate) async fn save_now<R>(app: AppHandle<R>, id: String) -> Result<()>
where
  R: Runtime,
{
  app.store_collection().save_now(id)
}

#[tauri::command]
pub(crate) async fn save_some<R>(app: AppHandle<R>, ids: Vec<String>) -> Result<()>
where
  R: Runtime,
{
  app.store_collection().save_some(&ids)
}

#[tauri::command]
pub(crate) async fn save_some_now<R>(app: AppHandle<R>, ids: Vec<String>) -> Result<()>
where
  R: Runtime,
{
  app.store_collection().save_some_now(&ids)
}

#[tauri::command]
pub(crate) async fn set_autosave<R>(app: AppHandle<R>, interval: u64)
where
  R: Runtime,
{
  app
    .store_collection()
    .set_autosave(Duration::from_millis(interval));
}

#[tauri::command]
pub(crate) async fn set_store_collection_path<R>(app: AppHandle<R>, path: PathBuf) -> Result<()>
where
  R: Runtime,
{
  spawn_blocking(move || app.store_collection().set_path(path)).await?
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
pub(crate) async fn unload<R>(app: AppHandle<R>, id: String) -> Result<()>
where
  R: Runtime,
{
  app.store_collection().unload_store(&id)
}
