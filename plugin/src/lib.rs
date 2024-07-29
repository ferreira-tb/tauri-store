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
//! tauri-plugin-pinia = 0.3
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
//! 5. Start the plugin:
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
mod store;

pub use error::Error;
use error::Result;
pub use pinia::Pinia;
pub use serde_json::Value as Json;
use std::path::{Path, PathBuf};
pub use store::{Store, StoreState};
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, RunEvent, Runtime, WebviewWindow, Window};

#[cfg(feature = "async-pinia")]
use {std::future::Future, std::pin::Pin, std::time::Duration, tauri::async_runtime};

#[cfg(feature = "ahash")]
use ahash::{HashMap, HashMapExt, HashSet};
#[cfg(not(feature = "ahash"))]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "async-pinia")]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait ManagerExt<R: Runtime>: Manager<R> {
  fn pinia(&self) -> tauri::State<Pinia<R>> {
    self.state::<Pinia<R>>()
  }

  #[cfg(not(feature = "async-pinia"))]
  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> Result<T>,
  {
    self.pinia().with_store(self.app_handle(), id, f)
  }

  #[cfg(feature = "async-pinia")]
  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> BoxFuture<Result<T>>
  where
    F: FnOnce(&mut Store<R>) -> BoxFuture<Result<T>> + Send + 'static,
    T: Send + 'static,
  {
    let id = id.as_ref().to_owned();
    let app = self.app_handle().clone();
    async move { app.pinia().with_store(&app, id, f).await }.boxed()
  }

  #[cfg(not(feature = "async-pinia"))]
  fn save_store(&self, id: impl AsRef<str>) -> Result<()> {
    self.with_store(id, |store| store.save())
  }

  #[cfg(feature = "async-pinia")]
  async fn save_store(&self, id: impl AsRef<str>) -> Result<()> {
    self
      .with_store(id, |store| store.save().boxed())
      .await
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}

#[cfg(not(feature = "async-pinia"))]
#[tauri::command]
async fn load<R: Runtime>(app: AppHandle<R>, id: String) -> Result<StoreState> {
  app.with_store(id, |store| Ok(store.state.clone()))
}

#[cfg(feature = "async-pinia")]
#[tauri::command]
async fn load<R: Runtime>(app: AppHandle<R>, id: String) -> Result<StoreState> {
  app
    .with_store(id, |store| async { Ok(store.state.clone()) }.boxed())
    .await
}

#[cfg(not(feature = "async-pinia"))]
#[tauri::command]
async fn patch<R: Runtime>(window: WebviewWindow<R>, id: String, state: StoreState) -> Result<()> {
  let app = window.app_handle().clone();
  app.with_store(id, move |store| {
    store.patch_with_source(state, window.label())
  })
}

#[cfg(feature = "async-pinia")]
#[tauri::command]
async fn patch<R: Runtime>(window: WebviewWindow<R>, id: String, state: StoreState) -> Result<()> {
  let app = window.app_handle().clone();
  app
    .with_store(id, move |store| {
      async move { store.patch_with_source(state, window.label()) }.boxed()
    })
    .await
}

#[cfg(not(feature = "async-pinia"))]
#[tauri::command]
async fn save<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.save_store(id)
}

#[cfg(feature = "async-pinia")]
#[tauri::command]
async fn save<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.save_store(id).await
}

#[cfg(not(feature = "async-pinia"))]
#[tauri::command]
async fn save_all<R: Runtime>(app: AppHandle<R>) {
  app.pinia().save_all();
}

#[cfg(feature = "async-pinia")]
#[tauri::command]
async fn save_all<R: Runtime>(app: AppHandle<R>) {
  app.pinia().save_all().await;
}

#[cfg(not(feature = "async-pinia"))]
#[tauri::command]
async fn unload<R: Runtime>(app: AppHandle<R>, id: String) {
  app.pinia().unload_store(&id);
}

#[cfg(feature = "async-pinia")]
#[tauri::command]
async fn unload<R: Runtime>(app: AppHandle<R>, id: String) {
  app.pinia().unload_store(&id).await;
}

#[derive(Default)]
pub struct Builder {
  path: Option<PathBuf>,
  sync_denylist: HashSet<String>,

  #[cfg(feature = "async-pinia")]
  autosave: Option<Duration>,
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

  /// Sets the autosave interval for all stores.
  #[cfg(feature = "async-pinia")]
  #[must_use]
  pub fn autosave(mut self, interval: Duration) -> Self {
    self.autosave = Some(interval);
    self
  }

  pub fn build<R: Runtime>(mut self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("pinia")
      .invoke_handler(tauri::generate_handler![
        load, patch, save, save_all, unload
      ])
      .setup(move |app, _| {
        let path = self.path.take().unwrap_or_else(|| {
          app
            .path()
            .app_data_dir()
            .expect("failed to resolve app data dir")
            .join("pinia")
        });

        #[cfg(feature = "tracing")]
        tracing::info!("pinia path: {}", path.display());

        app.manage(Pinia::<R> {
          path,
          sync_denylist: self.sync_denylist,

          #[cfg(not(feature = "async-pinia"))]
          stores: std::sync::Mutex::new(HashMap::new()),
          #[cfg(feature = "async-pinia")]
          stores: tokio::sync::Mutex::new(HashMap::new()),

          #[cfg(feature = "async-pinia")]
          autosave: std::sync::Mutex::new(None),
        });

        #[cfg(feature = "async-pinia")]
        if let Some(duration) = self.autosave {
          app.pinia().set_autosave(app, duration);
        };

        Ok(())
      })
      .on_event(|app, event| {
        if let RunEvent::Exit = event {
          #[cfg(not(feature = "async-pinia"))]
          app.pinia().save_all();
          #[cfg(feature = "async-pinia")]
          async_runtime::block_on(app.pinia().save_all());
        }
      })
      .build()
  }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}

#[cfg(feature = "async-pinia")]
pub trait FutureExt: Future {
  fn boxed<'a>(self) -> BoxFuture<'a, Self::Output>
  where
    Self: Sized + Send + 'a,
  {
    Box::pin(self)
  }
}

#[cfg(feature = "async-pinia")]
impl<T> FutureExt for T where T: ?Sized + Future {}
