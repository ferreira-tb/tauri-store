#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod command;
mod manager;
mod pinia;

use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, RunEvent, Runtime};
use tauri_store::CollectionBuilder;

pub use manager::ManagerExt;
pub use pinia::Pinia;
pub use tauri_store::{
  with_store, BoxResult, Error, Json, OnLoadFn, OnLoadResult, Result, SaveStrategy, Store,
  StoreOptions, StoreState, StoreStateExt, WatcherResult,
};

#[cfg(feature = "unstable-async")]
use tauri::async_runtime::block_on;

#[cfg(feature = "unstable-async")]
pub use tauri_store::boxed;

/// Builder for the Pinia plugin.
#[derive(CollectionBuilder)]
pub struct Builder<R: Runtime> {
  path: Option<PathBuf>,
  default_save_strategy: SaveStrategy,
  autosave: Option<Duration>,
  on_load: Option<Box<OnLoadFn<R>>>,
  pretty: bool,
  save_denylist: HashSet<String>,
  sync_denylist: HashSet<String>,
}

impl<R: Runtime> Builder<R> {
  // This only exists for backward compatibility.
  // New plugins should use their full name as the directory.
  const STORE_DIR: &'static str = "pinia";

  /// Builds the plugin.
  pub fn build(self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("pinia")
      .setup(|app, _| setup(app, self))
      .on_event(on_event)
      .invoke_handler(tauri::generate_handler![
        command::clear_autosave,
        command::get_default_save_strategy,
        command::get_pinia_path,
        command::get_save_strategy,
        command::get_store_ids,
        command::get_store_path,
        command::get_store_state,
        command::load,
        command::patch,
        command::save,
        command::save_all,
        command::save_all_now,
        command::save_now,
        command::save_some,
        command::save_some_now,
        command::set_autosave,
        command::set_save_strategy,
        command::set_store_options,
        command::unload
      ])
      .build()
  }
}

#[expect(clippy::unnecessary_wraps)]
fn setup<R: Runtime>(app: &AppHandle<R>, builder: Builder<R>) -> BoxResult<()> {
  let collection = builder.into_collection(app);
  app.manage(Pinia(collection));

  Ok(())
}

fn on_event<R: Runtime>(app: &AppHandle<R>, event: &RunEvent) {
  if let RunEvent::Exit = event {
    let pinia = app.pinia();
    #[cfg(not(feature = "unstable-async"))]
    let _ = pinia.save_all_now();
    #[cfg(feature = "unstable-async")]
    let _ = block_on(pinia.save_all_now());
  }
}

/// Initializes the plugin with the default settings.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}
