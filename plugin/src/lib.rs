#![cfg(not(any(target_os = "android", target_os = "ios")))]

mod error;
mod pinia;
pub mod prelude;
mod store;

pub use error::Error;
use error::Result;
pub use pinia::Pinia;
pub use serde_json::Value as Json;
use std::path::PathBuf;
use std::sync::Mutex;
pub use store::{State, Store};
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, RunEvent, Runtime, WebviewWindow};

#[cfg(feature = "ahash")]
use ahash::{HashMap, HashMapExt};
#[cfg(not(feature = "ahash"))]
use std::collections::HashMap;

pub trait AppHandleExt<R: Runtime>: Manager<R> {
  fn pinia(&self) -> tauri::State<Pinia<R>> {
    self.state::<Pinia<R>>()
  }

  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> Result<T>,
  {
    self.pinia().with_store(self.app_handle(), id, f)
  }
}

impl<R: Runtime> AppHandleExt<R> for AppHandle<R> {}

#[tauri::command]
async fn load<R: Runtime>(app: AppHandle<R>, id: String) -> Result<State> {
  app.with_store(id, |store| Ok(store.state.clone()))
}

#[tauri::command]
async fn save<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.with_store(id, move |store| store.save())
}

#[tauri::command]
async fn save_all<R: Runtime>(app: AppHandle<R>) {
  app.pinia().save();
}

#[tauri::command]
async fn set<R: Runtime>(window: WebviewWindow<R>, id: String, state: State) -> Result<()> {
  let app = window.app_handle().clone();
  app.with_store(id, move |store| store.set(state, window.label()))
}

#[derive(Default)]
pub struct Builder {
  path: Option<PathBuf>,
}

impl Builder {
  pub fn new() -> Self {
    Self::default()
  }

  /// Directory where the stores will be saved.
  pub fn path(mut self, path: PathBuf) -> Self {
    self.path = Some(path);
    self
  }

  pub fn build<R: Runtime>(mut self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("pinia")
      .invoke_handler(tauri::generate_handler![load, save, save_all, set])
      .setup(move |app, _| {
        let path = self.path.take().unwrap_or_else(|| {
          app
            .path()
            .app_data_dir()
            .expect("failed to resolve app data dir")
            .join("pinia")
        });

        app.manage(Pinia::<R> {
          path,
          stores: Mutex::new(HashMap::new()),
        });

        Ok(())
      })
      .on_event(|app, event| {
        if matches!(event, RunEvent::Exit) {
          app.pinia().save();
        }
      })
      .build()
  }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}
