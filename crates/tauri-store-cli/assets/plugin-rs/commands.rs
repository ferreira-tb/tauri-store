use crate::manager::ManagerExt;
use __IMPORT_SOURCE__::{Result, SaveStrategy, StoreId, StoreOptions, StoreState};
use std::path::PathBuf;
use std::time::Duration;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow};

#[tauri::command]
pub(crate) async fn allow_save<R>(app: AppHandle<R>, ids: Vec<StoreId>)
where
  R: Runtime,
{
  let __STORE_COLLECTION__ = app.__STORE_COLLECTION__();
  ids
    .iter()
    .for_each(|id| __STORE_COLLECTION__.allow_save(id));
}

#[tauri::command]
pub(crate) async fn allow_sync<R>(app: AppHandle<R>, ids: Vec<StoreId>)
where
  R: Runtime,
{
  let __STORE_COLLECTION__ = app.__STORE_COLLECTION__();
  ids
    .iter()
    .for_each(|id| __STORE_COLLECTION__.allow_sync(id));
}

#[tauri::command]
pub(crate) async fn clear_autosave<R>(app: AppHandle<R>)
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().clear_autosave();
}

#[tauri::command]
pub(crate) async fn deny_save<R>(app: AppHandle<R>, ids: Vec<StoreId>)
where
  R: Runtime,
{
  let __STORE_COLLECTION__ = app.__STORE_COLLECTION__();
  ids
    .iter()
    .for_each(|id| __STORE_COLLECTION__.deny_save(id));
}

#[tauri::command]
pub(crate) async fn deny_sync<R>(app: AppHandle<R>, ids: Vec<StoreId>)
where
  R: Runtime,
{
  let __STORE_COLLECTION__ = app.__STORE_COLLECTION__();
  ids
    .iter()
    .for_each(|id| __STORE_COLLECTION__.deny_sync(id));
}

#[tauri::command]
pub(crate) async fn get_default_save_strategy<R>(app: AppHandle<R>) -> SaveStrategy
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().default_save_strategy()
}

#[tauri::command]
pub(crate) async fn get_store_collection_path<R>(app: AppHandle<R>) -> PathBuf
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().path()
}

#[tauri::command]
pub(crate) async fn get_store_ids<R>(app: AppHandle<R>) -> Vec<StoreId>
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().ids()
}

#[tauri::command]
pub(crate) async fn get_store_path<R>(app: AppHandle<R>, id: StoreId) -> Result<PathBuf>
where
  R: Runtime,
{
  app
    .__STORE_COLLECTION__()
    .with_store(id, |store| store.path())
}

#[tauri::command]
pub(crate) async fn get_save_strategy<R>(app: AppHandle<R>, id: StoreId) -> Result<SaveStrategy>
where
  R: Runtime,
{
  app
    .__STORE_COLLECTION__()
    .with_store(id, |store| store.save_strategy())
}

#[tauri::command]
pub(crate) async fn get_store_state<R>(app: AppHandle<R>, id: StoreId) -> Result<StoreState>
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().state(id)
}

#[tauri::command]
pub(crate) async fn load<R>(app: AppHandle<R>, id: StoreId) -> Result<StoreState>
where
  R: Runtime,
{
  spawn_blocking(move || {
    app
      .__STORE_COLLECTION__()
      .with_store(id, |store| store.state().clone())
  })
  .await?
}

#[tauri::command]
pub(crate) async fn patch<R>(window: WebviewWindow<R>, id: StoreId, state: StoreState) -> Result<()>
where
  R: Runtime,
{
  let app = window.app_handle();
  let label = window.label().to_owned();
  app
    .__STORE_COLLECTION__()
    .with_store(id, move |store| store.patch_with_source(state, label))?
}

#[tauri::command]
pub(crate) async fn save<R>(app: AppHandle<R>, id: StoreId) -> Result<()>
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().save(id)
}

#[tauri::command]
pub(crate) async fn save_all<R>(app: AppHandle<R>) -> Result<()>
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().save_all()
}

#[tauri::command]
pub(crate) async fn save_all_now<R>(app: AppHandle<R>) -> Result<()>
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().save_all_now()
}

#[tauri::command]
pub(crate) async fn save_now<R>(app: AppHandle<R>, id: StoreId) -> Result<()>
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().save_now(id)
}

#[tauri::command]
pub(crate) async fn save_some<R>(app: AppHandle<R>, ids: Vec<StoreId>) -> Result<()>
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().save_some(&ids)
}

#[tauri::command]
pub(crate) async fn save_some_now<R>(app: AppHandle<R>, ids: Vec<StoreId>) -> Result<()>
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().save_some_now(&ids)
}

#[tauri::command]
pub(crate) async fn set_autosave<R>(app: AppHandle<R>, interval: u64)
where
  R: Runtime,
{
  app
    .__STORE_COLLECTION__()
    .set_autosave(Duration::from_millis(interval));
}

#[tauri::command]
pub(crate) async fn set_store_collection_path<R>(app: AppHandle<R>, path: PathBuf) -> Result<()>
where
  R: Runtime,
{
  spawn_blocking(move || app.__STORE_COLLECTION__().set_path(path)).await?
}

#[tauri::command]
pub(crate) async fn set_save_strategy<R>(
  app: AppHandle<R>,
  id: StoreId,
  strategy: SaveStrategy,
) -> Result<()>
where
  R: Runtime,
{
  app
    .__STORE_COLLECTION__()
    .with_store(id, |store| store.set_save_strategy(strategy))
}

#[tauri::command]
pub(crate) async fn set_store_options<R>(
  window: WebviewWindow<R>,
  id: StoreId,
  options: StoreOptions,
) -> Result<()>
where
  R: Runtime,
{
  let app = window.app_handle();
  let label = window.label().to_owned();
  app
    .__STORE_COLLECTION__()
    .with_store(id, move |store| {
      store.set_options_with_source(options, label)
    })?
}

#[tauri::command]
pub(crate) async fn unload<R>(app: AppHandle<R>, id: StoreId) -> Result<()>
where
  R: Runtime,
{
  app.__STORE_COLLECTION__().unload_store(&id)
}
