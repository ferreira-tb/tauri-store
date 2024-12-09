#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod command;
mod manager;
mod pinia;

use std::path::PathBuf;
use std::time::Duration;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, RunEvent, Runtime};
use tauri_store::{CollectionBuilder, StoreCollection};

pub use manager::ManagerExt;
pub use pinia::Pinia;
pub use tauri_store::{
  with_store, BoxResult, Error, Json, OnLoadFn, OnLoadResult, Result, SaveStrategy, Store,
  StoreState, StoreStateExt, WatcherResult,
};

#[cfg(feature = "unstable-async")]
use tauri::async_runtime::block_on;

#[cfg(feature = "unstable-async")]
pub use tauri_store::boxed;

#[cfg(feature = "ahash")]
use ahash::HashSet;
#[cfg(not(feature = "ahash"))]
use std::collections::HashSet;

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
  /// Builds the plugin.
  pub fn build(self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("pinia")
      .setup(|app, _| setup(app, self))
      .on_event(on_event)
      .invoke_handler(tauri::generate_handler![
        command::clear_autosave,
        command::get_default_save_strategy,
        command::get_pinia_path,
        command::get_store_ids,
        command::get_store_path,
        command::get_store_save_strategy,
        command::get_store_state,
        command::load,
        command::patch,
        command::save,
        command::save_all,
        command::save_some,
        command::set_autosave,
        command::set_store_save_strategy,
        command::unload
      ])
      .build()
  }
}

impl<R: Runtime> Default for Builder<R> {
  fn default() -> Self {
    Self {
      path: None,
      default_save_strategy: SaveStrategy::default(),
      autosave: None,
      on_load: None,
      pretty: false,
      save_denylist: HashSet::default(),
      sync_denylist: HashSet::default(),
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
    .default_save_strategy(builder.default_save_strategy)
    .pretty(builder.pretty)
    .save_denylist(builder.save_denylist)
    .sync_denylist(builder.sync_denylist);

  if let Some(on_load) = builder.on_load {
    collection = collection.on_load(on_load);
  }

  if let Some(duration) = builder.autosave {
    collection = collection.autosave(duration);
  };

  app.manage(Pinia(collection.build(app)));

  Ok(())
}

fn on_event<R: Runtime>(app: &AppHandle<R>, event: &RunEvent) {
  if let RunEvent::Exit = event {
    let pinia = app.pinia();
    #[cfg(not(feature = "unstable-async"))]
    let _ = pinia.save_all();
    #[cfg(feature = "unstable-async")]
    let _ = block_on(pinia.save_all());
  }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}
