#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://tb.dev.br/tauri-store/favicon.ico")]

mod __SNAKE_PLUGIN_NAME__;
mod command;
mod manager;

use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, RunEvent, Runtime};
use tauri_store::CollectionBuilder;

pub use manager::ManagerExt;
pub use tauri_store::prelude::*;
pub use __SNAKE_PLUGIN_NAME__::__PASCAL_PLUGIN_NAME__;

/// Builder for the __PASCAL_PLUGIN_NAME__ plugin.
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
  /// Builds the __PASCAL_PLUGIN_NAME__ plugin.
  pub fn build(self) -> TauriPlugin<R> {
    tauri::plugin::Builder::new("__REGISTERED_PLUGIN_NAME__")
      .setup(|app, _| setup(app, self))
      .on_event(on_event)
      .invoke_handler(tauri::generate_handler![
        command::clear_autosave,
        command::get_default_save_strategy,
        command::get___STORE_COLLECTION___path,
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
        command::set___STORE_COLLECTION___path,
        command::set_store_options,
        command::unload
      ])
      .build()
  }
}

#[allow(clippy::unnecessary_wraps)]
fn setup<R: Runtime>(app: &AppHandle<R>, builder: Builder<R>) -> BoxResult<()> {
  let collection = builder.into_collection(app);
  app.manage(__PASCAL_PLUGIN_NAME__(collection));

  Ok(())
}

fn on_event<R: Runtime>(app: &AppHandle<R>, event: &RunEvent) {
  if let RunEvent::Exit = event {
    let _ = app.__SNAKE_PLUGIN_NAME__().0.on_exit();
  }
}

/// Initializes the plugin with the default settings.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}
