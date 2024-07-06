use serde::{Serialize, Serializer};
use std::path::{Path, PathBuf};
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, Runtime};
pub use tauri_plugin_store::JsonValue as Json;
use tauri_plugin_store::{Error as StoreError, Store, StoreCollection};

type Result<T> = std::result::Result<T, Error>;
type StoreResult<T> = std::result::Result<T, StoreError>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Store(#[from] tauri_plugin_store::Error),
  #[error(transparent)]
  Tauri(#[from] tauri::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}

pub trait TauriStore<R: Runtime>: Manager<R> {
  fn with_store<F, T>(&self, path: impl AsRef<Path>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> StoreResult<T>,
  {
    let app = self.app_handle().clone();
    let collection = self.state::<StoreCollection<R>>();
    tauri_plugin_store::with_store(app, collection, path, f).map_err(Into::into)
  }
}

impl<R: Runtime> TauriStore<R> for AppHandle<R> {}

#[tauri::command]
async fn set<R: Runtime>(app: AppHandle<R>, path: PathBuf, key: String, value: Json) -> Result<()> {
  app.with_store(path, |store| store.insert(key, value))
}

#[tauri::command]
async fn entries<R: Runtime>(app: AppHandle<R>, path: PathBuf) -> Result<Vec<(String, Json)>> {
  app.with_store(path, |store| {
    let entries = store
      .entries()
      .map(|(k, v)| (k.to_owned(), v.to_owned()))
      .collect();

    Ok(entries)
  })
}

#[tauri::command]
async fn load<R: Runtime>(app: AppHandle<R>, path: PathBuf) -> Result<()> {
  app.with_store(path, |store| {
    let result = store.load();
    if matches!(result, Err(StoreError::NotFound(_))) {
      return Ok(());
    }

    result
  })
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  tauri::plugin::Builder::new("pinia")
    .invoke_handler(tauri::generate_handler![entries, load, set])
    .build()
}
