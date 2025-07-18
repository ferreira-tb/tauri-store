#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://tb.dev.br/tauri-store/favicon.ico")]

mod __SNAKE_PLUGIN_TITLE__;
mod command;
mod manager;

use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, RunEvent, Runtime};
use tauri_store::CollectionBuilder;

#[cfg(feature = "unstable-migration")]
use tauri_store::Migrator;

pub use __SNAKE_PLUGIN_TITLE__::{__PASCAL_PLUGIN_TITLE__, __PASCAL_PLUGIN_TITLE__Marker};
pub use manager::ManagerExt;
pub use tauri_store::prelude::*;

#[cfg(feature = "unstable-migration")]
pub use tauri_store::{Migration, MigrationContext};

// The `CollectionBuilder` macro depends on this.
type Marker = __PASCAL_PLUGIN_TITLE__Marker;

/// Builder for the __PASCAL_PLUGIN_TITLE__ plugin.
#[derive(CollectionBuilder)]
pub struct Builder<R: Runtime> {
  path: Option<PathBuf>,
  default_save_strategy: SaveStrategy,
  autosave: Option<Duration>,
  on_load: Option<Box<OnLoadFn<R, Marker>>>,
  pretty: bool,
  save_denylist: HashSet<StoreId>,
  sync_denylist: HashSet<StoreId>,

  #[cfg(feature = "unstable-migration")]
  migrator: Migrator,
}

impl<R: Runtime> Builder<R> {
  /// Builds the __PASCAL_PLUGIN_TITLE__ plugin.
  pub fn build(self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("__REGISTERED_PLUGIN_NAME__")
      .setup(|app, _| setup(app, self))
      .on_event(on_event)
      .invoke_handler(tauri::generate_handler![
        command::allow_save,
        command::allow_sync,
        command::clear_autosave,
        command::deny_save,
        command::deny_sync,
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
  builder.build_collection(app)?;
  Ok(())
}

fn on_event<R>(app: &AppHandle<R>, event: &RunEvent)
where
  R: Runtime,
{
  if let RunEvent::Exit = event {
    let _ = app.__SNAKE_PLUGIN_TITLE__().0.on_exit();
  }
}

/// Initializes the plugin with the default settings.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}
