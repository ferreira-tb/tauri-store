//! # tauri-plugin-pinia
//!
//! Persistent Pinia stores for Tauri.
//!
//! ## Features
//!
//! - Save your Pinia stores to disk on app exit (or manually, if needed).
//! - Synchronize your stores across multiple windows.
//! - Debounce store updates.
//!
//! ## Install
//!
//! Install the Rust crate by adding the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! tauri-plugin-pinia = 0.2
//! ```
//!
//! Install the JavaScript package with your preferred package manager:
//!
//! ```sh
//! pnpm add tauri-plugin-pinia
//! ```
//!
//! ## Usage
//!
//! > For a working example, see the [playground](https://github.com/ferreira-tb/tauri-plugin-pinia/tree/main/packages/playground).
//!
//! 1. Enable the required permissions in your capabilities file:
//!
//! `src-tauri/capabilities/pinia.json`
//!
//! ```json
//! {
//!   "identifier": "pinia",
//!   "windows": ["*"],
//!   "permissions": ["pinia:default", "event:allow-listen", "event:allow-unlisten"]
//! }
//! ```
//!
//! 2. Register the plugin with Tauri:
//!
//! `src-tauri/src/main.rs`
//!
//! ```rust
//! tauri::Builder::default()
//!   .plugin(tauri_plugin_pinia::init())
//!   .run(tauri::generate_context!())
//!   .expect("error while running tauri application");
//!
//! ```
//!
//! 3. Enable the plugin for Pinia:
//!
//! `src/index.ts`
//!
//! ```ts
//! import { createApp } from 'vue';
//! import { createPinia } from 'pinia';
//! import { createPlugin } from 'tauri-plugin-pinia';
//!
//! const app = createApp(App);
//!
//! const pinia = createPinia();
//! pinia.use(createPlugin());
//!
//! app.use(pinia).mount('#app');
//! ```
//!
//! 4. Create your Pinia store:
//!
//! `src/stores/counter.ts`
//!
//! ```ts
//! import { ref } from 'vue';
//! import { defineStore } from 'pinia';
//!
//! export const useCounterStore = defineStore('counter', () => {
//!   const counter = ref(0);
//!
//!   function increment() {
//!     counter.value++;
//!   }
//!
//!   return {
//!     counter,
//!     increment,
//!   };
//! });
//! ```
//!
//! 5. Start the plugin!
//!
//! ```ts
//! import { useCounterStore } from './stores/counter';
//!
//! const counterStore = useCounterStore();
//! counterStore.$tauri.start();
//! ```

#![forbid(unsafe_code)]
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
pub use store::{Store, StoreState};
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, RunEvent, Runtime, WebviewWindow, Window};

#[cfg(feature = "ahash")]
use ahash::{HashMap, HashMapExt, HashSet};
#[cfg(not(feature = "ahash"))]
use std::collections::{HashMap, HashSet};

pub trait ManagerExt<R: Runtime>: Manager<R> {
  fn pinia(&self) -> tauri::State<Pinia<R>> {
    self.state::<Pinia<R>>()
  }

  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> Result<T>,
  {
    self.pinia().with_store(self.app_handle(), id, f)
  }

  fn save_store(&self, id: impl AsRef<str>) -> Result<()> {
    self.with_store(id, |store| store.save())
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}

#[tauri::command]
async fn load<R: Runtime>(app: AppHandle<R>, id: String) -> Result<StoreState> {
  app.with_store(id, |store| Ok(store.state.clone()))
}

#[tauri::command]
async fn patch<R: Runtime>(window: WebviewWindow<R>, id: String, state: StoreState) -> Result<()> {
  let app = window.app_handle().clone();
  app.with_store(id, move |store| {
    store.patch_with_source(state, window.label())
  })
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
  #[must_use]
  pub fn path(mut self, path: impl AsRef<Path>) -> Self {
    let path = path.as_ref().to_path_buf();
    self.path = Some(path);
    self
  }

  /// Sets a list of stores that should not be synchronized across windows.
  #[must_use]
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
