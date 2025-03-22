// This file was autogenerated and should not be edited manually.
// Check the `codegen` command in the `tauri-store-cli` crate.

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://tb.dev.br/tauri-store/favicon.ico")]

mod command;
mod manager;
mod valtio;

use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, RunEvent, Runtime};
use tauri_store::CollectionBuilder;

#[cfg(feature = "unstable-migration")]
use tauri_store::Migrator;

pub use manager::ManagerExt;
pub use tauri_store::prelude::*;
pub use valtio::Valtio;

#[cfg(feature = "unstable-migration")]
pub use tauri_store::{Migration, MigrationContext};

/// Builder for the Valtio plugin.
#[derive(CollectionBuilder)]
pub struct Builder<R: Runtime> {
  path: Option<PathBuf>,
  default_save_strategy: SaveStrategy,
  autosave: Option<Duration>,
  on_load: Option<Box<OnLoadFn<R>>>,
  pretty: bool,
  save_denylist: HashSet<StoreId>,
  sync_denylist: HashSet<StoreId>,

  #[cfg(feature = "unstable-migration")]
  migrator: Migrator,
}

impl<R: Runtime> Builder<R> {
  /// Builds the Valtio plugin.
  pub fn build(self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("valtio")
      .setup(|app, _| setup(app, self))
      .on_event(on_event)
      .invoke_handler(tauri::generate_handler![
        command::clear_autosave,
        command::get_default_save_strategy,
        command::get_store_collection_path,
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
        command::set_store_collection_path,
        command::set_store_options,
        command::unload
      ])
      .build()
  }
}

fn setup<R>(app: &AppHandle<R>, builder: Builder<R>) -> BoxResult<()>
where
  R: Runtime,
{
  let collection = builder.build_collection(app)?;
  app.manage(Valtio(collection));

  Ok(())
}

fn on_event<R>(app: &AppHandle<R>, event: &RunEvent)
where
  R: Runtime,
{
  if let RunEvent::Exit = event {
    let _ = app.valtio().0.on_exit();
  }
}

/// Initializes the plugin with the default settings.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}
