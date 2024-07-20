//! Persistent Pinia stores for Tauri.
//! 
//! ## Features
//! 
//! - Saves your Pinia stores to disk on application exit (or manually, if needed).
//! - Synchronizes your stores across multiple windows.
//! - Allows debouncing store updates.

#![cfg(not(any(target_os = "android", target_os = "ios")))]

mod error;
mod pinia;
pub mod prelude;
mod store;

pub use error::Error;
use error::Result;
pub use pinia::Pinia;
pub use serde_json::Value as Json;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
pub use store::{State, Store};
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, RunEvent, Runtime, WebviewWindow, Window};

#[cfg(feature = "ahash")]
use ahash::{HashMap, HashMapExt, HashSet};
#[cfg(not(feature = "ahash"))]
use std::collections::{HashMap, HashSet};

pub trait PiniaExt<R: Runtime>: Manager<R> {
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

impl<R: Runtime> PiniaExt<R> for AppHandle<R> {}
impl<R: Runtime> PiniaExt<R> for Window<R> {}
impl<R: Runtime> PiniaExt<R> for WebviewWindow<R> {}

#[tauri::command]
async fn load<R: Runtime>(app: AppHandle<R>, id: String) -> Result<State> {
  app.with_store(id, |store| Ok(store.state.clone()))
}

#[tauri::command]
async fn patch<R: Runtime>(window: WebviewWindow<R>, id: String, state: State) -> Result<()> {
  let app = window.app_handle().clone();
  app.with_store(id, move |store| store.patch(state, window.label()))
}

#[tauri::command]
async fn save<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.with_store(id, move |store| store.save())
}

#[tauri::command]
async fn save_all<R: Runtime>(app: AppHandle<R>) {
  app.pinia().save();
}

#[derive(Default)]
pub struct Builder {
  path: Option<PathBuf>,
  sync_denylist: HashSet<String>,
}

impl Builder {
  pub fn new() -> Self {
    Self::default()
  }

  /// Directory where the stores will be saved.
  pub fn path(mut self, path: impl AsRef<Path>) -> Self {
    let path = path.as_ref().to_path_buf();
    self.path = Some(path);
    self
  }

  /// Sets a list of stores that should not be synchronized across windows.
  pub fn sync_denylist(mut self, denylist: &[&str]) -> Self {
    self
      .sync_denylist
      .extend(denylist.iter().map(ToString::to_string));

    self
  }

  pub fn build<R: Runtime>(mut self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("pinia")
      .invoke_handler(tauri::generate_handler![load, patch, save, save_all])
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
          sync_denylist: self.sync_denylist,
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
