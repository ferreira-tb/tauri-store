//! # tauri-plugin-pinia
//!
//! Persistent Pinia stores for Tauri and Vue.
//!
//! ## Features
//!
//! - Save your stores to disk.
//! - Synchronize across multiple windows.
//! - Debounce or throttle store updates.
//!
//! ## Documentation
//!
//! Check the [documentation](https://tb.dev.br/tauri-store/pinia/getting-started.html) for more information on how to install and use the plugin.
//!
//! ## Supported Tauri Version
//!
//! This plugin requires Tauri `2.0` or later.
//!

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod command;
mod manager;
mod pinia;

pub use manager::ManagerExt;
pub use pinia::Pinia;
use std::path::{Path, PathBuf};
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, RunEvent, Runtime};
use tauri_store::StoreCollection;
pub use tauri_store::{
  BoxResult, Error, Json, OnLoadFn, OnLoadResult, Result, Store, StoreState, StoreStateExt,
  WatcherResult,
};

#[cfg(feature = "unstable-async")]
pub use tauri_store::{boxed, boxed_ok, BoxFuture};

#[cfg(feature = "unstable-async")]
use {std::time::Duration, tauri::async_runtime};

#[cfg(feature = "ahash")]
use ahash::HashSet;
#[cfg(not(feature = "ahash"))]
use std::collections::HashSet;

pub struct Builder<R: Runtime> {
  path: Option<PathBuf>,
  pretty: bool,
  save_denylist: HashSet<String>,
  sync_denylist: HashSet<String>,

  on_load: Option<Box<OnLoadFn<R>>>,

  #[cfg(feature = "unstable-async")]
  autosave: Option<Duration>,
}

impl<R: Runtime> Builder<R> {
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

  /// Sets whether the store files should be pretty printed.
  #[must_use]
  pub fn pretty(mut self, yes: bool) -> Self {
    self.pretty = yes;
    self
  }

  /// Sets a list of stores that should not be saved to disk.
  #[must_use]
  pub fn save_denylist(mut self, denylist: &[impl AsRef<str>]) -> Self {
    self
      .save_denylist
      .extend(denylist.iter().map(|s| s.as_ref().to_string()));

    self
  }

  /// Sets a list of stores that should not be synchronized across windows.
  #[must_use]
  pub fn sync_denylist(mut self, denylist: &[impl AsRef<str>]) -> Self {
    self
      .sync_denylist
      .extend(denylist.iter().map(|s| s.as_ref().to_string()));

    self
  }

  /// Sets a function to be called when a store is loaded.
  #[must_use]
  pub fn on_load<F>(mut self, f: F) -> Self
  where
    F: Fn(&Store<R>) -> OnLoadResult + Send + Sync + 'static,
  {
    self.on_load = Some(Box::new(f));
    self
  }

  /// Sets the autosave interval for all stores.
  #[cfg(feature = "unstable-async")]
  #[cfg_attr(docsrs, doc(cfg(feature = "unstable-async")))]
  #[must_use]
  pub fn autosave(mut self, interval: Duration) -> Self {
    self.autosave = Some(interval);
    self
  }

  pub fn build(self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("pinia")
      .setup(|app, _| setup(app, self))
      .on_event(on_event)
      .invoke_handler(tauri::generate_handler![
        command::clear_autosave,
        command::get_pinia_path,
        command::get_store_ids,
        command::get_store_path,
        command::get_store_state,
        command::load,
        command::patch,
        command::save,
        command::save_all,
        command::save_some,
        command::set_autosave,
        command::unload
      ])
      .build()
  }
}

impl<R: Runtime> Default for Builder<R> {
  fn default() -> Self {
    Self {
      path: None,
      pretty: false,
      save_denylist: HashSet::default(),
      sync_denylist: HashSet::default(),
      on_load: None,

      #[cfg(feature = "unstable-async")]
      autosave: None,
    }
  }
}

#[expect(clippy::unnecessary_wraps)]
fn setup<R: Runtime>(app: &AppHandle<R>, mut builder: Builder<R>) -> BoxResult<()> {
  let path = builder.path.take().unwrap_or_else(|| {
    app
      .path()
      .app_data_dir()
      .expect("failed to resolve app data dir")
      .join("pinia")
  });

  let mut collection = StoreCollection::<R>::builder()
    .path(path)
    .pretty(builder.pretty)
    .save_denylist(builder.save_denylist)
    .sync_denylist(builder.sync_denylist);

  if let Some(on_load) = builder.on_load {
    collection = collection.on_load(on_load);
  }

  app.manage(Pinia(collection.build(app)));

  #[cfg(feature = "unstable-async")]
  if let Some(duration) = builder.autosave {
    app.pinia().set_autosave(duration);
  };

  Ok(())
}

fn on_event<R: Runtime>(app: &AppHandle<R>, event: &RunEvent) {
  if let RunEvent::Exit = event {
    #[cfg(not(feature = "unstable-async"))]
    let _ = app.pinia().save_all();
    #[cfg(feature = "unstable-async")]
    let _ = async_runtime::block_on(app.pinia().save_all());
  }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}
