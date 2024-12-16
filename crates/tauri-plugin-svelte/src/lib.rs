#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod command;
mod manager;
mod svelte;

use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, RunEvent, Runtime};
use tauri_store::CollectionBuilder;

pub use manager::ManagerExt;
pub use svelte::Svelte;
pub use tauri_store::{
  with_store, BoxResult, Error, Json, OnLoadFn, OnLoadResult, Result, SaveStrategy, Store,
  StoreOptions, StoreState, StoreStateExt, WatcherResult,
};

#[cfg(feature = "unstable-async")]
use tauri::async_runtime::block_on;

#[cfg(feature = "unstable-async")]
pub use tauri_store::boxed;

/// Builder for the Svelte plugin.
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
  const STORE_DIR: &'static str = env!("CARGO_PKG_NAME");

  /// Builds the plugin.
  pub fn build(self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("svelte")
      .setup(|app, _| setup(app, self))
      .on_event(on_event)
      .invoke_handler(tauri::generate_handler![
        command::clear_autosave,
        command::get_default_save_strategy,
        command::get_save_strategy,
        command::get_store_ids,
        command::get_store_path,
        command::get_store_state,
        command::get_svelte_path,
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
  app.manage(Svelte(collection));

  Ok(())
}

fn on_event<R: Runtime>(app: &AppHandle<R>, event: &RunEvent) {
  if let RunEvent::Exit = event {
    let svelte = app.svelte();
    #[cfg(not(feature = "unstable-async"))]
    let _ = svelte.save_all_now();
    #[cfg(feature = "unstable-async")]
    let _ = block_on(svelte.save_all_now());
  }
}

/// Initializes the plugin with the default settings.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}
