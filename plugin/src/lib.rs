//! # tauri-plugin-pinia
//!
//! Persistent Pinia stores for Tauri and Vue.
//!
//! ## Features
//!
//! - Save your Pinia stores to disk.
//! - Synchronize your stores across multiple windows.
//! - Debounce store updates.
//!
//! ## Documentation
//!
//! Check the [documentation](https://tb.dev.br/tauri-plugin-pinia/getting-started.html) for more information on how to install and use the plugin.
//!
//! ## Supported Tauri Version
//!
//! This plugin requires Tauri `2.0.0-rc` or later.
//!

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod command;
mod error;
mod manager;
mod pinia;
mod store;

pub use error::{Error, Result};
pub use manager::ManagerExt;
pub use pinia::Pinia;
pub use serde_json::Value as Json;
use std::path::{Path, PathBuf};
pub use store::{Store, StoreState};
use tauri::plugin::TauriPlugin;
use tauri::{Manager, RunEvent, Runtime};

#[cfg(feature = "async-pinia")]
use {std::future::Future, std::pin::Pin, std::time::Duration, tauri::async_runtime};

#[cfg(feature = "ahash")]
use ahash::{HashMap, HashMapExt, HashSet};
#[cfg(not(feature = "ahash"))]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "async-pinia")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-pinia")))]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

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
  #[cfg_attr(docsrs, doc(cfg(feature = "async-pinia")))]
  #[must_use]
  pub fn autosave(mut self, interval: Duration) -> Self {
    self.autosave = Some(interval);
    self
  }

  pub fn build<R: Runtime>(mut self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("pinia")
      .invoke_handler(tauri::generate_handler![
        command::clear_autosave,
        command::get_pinia_path,
        command::get_store_ids,
        command::get_store_path,
        command::load,
        command::patch,
        command::save,
        command::save_all,
        command::save_some,
        command::set_autosave,
        command::unload
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
        tracing::trace!("pinia path: {}", path.display());

        app.manage(Pinia::<R> {
          app: app.clone(),
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
          app.pinia().set_autosave(duration);
        };

        Ok(())
      })
      .on_event(|app, event| {
        if let RunEvent::Exit = event {
          #[cfg(not(feature = "async-pinia"))]
          let _ = app.pinia().save_all();
          #[cfg(feature = "async-pinia")]
          let _ = async_runtime::block_on(app.pinia().save_all());
        }
      })
      .build()
  }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}

#[cfg(not(feature = "async-pinia"))]
pub fn with_store<R, M, F, T>(manager: &M, id: impl AsRef<str>, f: F) -> Result<T>
where
  R: Runtime,
  M: Manager<R> + ManagerExt<R>,
  F: FnOnce(&mut Store<R>) -> Result<T>,
{
  manager.pinia().with_store(id, f)
}

#[cfg(feature = "async-pinia")]
pub async fn with_store<R, M, F, T>(manager: &M, id: impl AsRef<str>, f: F) -> Result<T>
where
  R: Runtime,
  M: Manager<R> + ManagerExt<R>,
  F: FnOnce(&mut Store<R>) -> BoxFuture<Result<T>> + Send + 'static,
  T: Send + 'static,
{
  manager.pinia().with_store(id, f).await
}

#[cfg(feature = "async-pinia")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-pinia")))]
pub trait FutureExt: Future {
  /// Wrap the future in a Box, pinning it.
  fn boxed<'a>(self) -> BoxFuture<'a, Self::Output>
  where
    Self: Sized + Send + 'a,
  {
    Box::pin(self)
  }

  fn boxed_ok<'a>(self) -> BoxFuture<'a, Result<Self::Output>>
  where
    Self: Sized + Send + 'a,
  {
    Box::pin(async move { Ok(self.await) })
  }
}

#[cfg(feature = "async-pinia")]
impl<T> FutureExt for T where T: ?Sized + Future {}
