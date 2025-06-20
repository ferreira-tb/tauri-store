use crate::collection::DefaultMarker;
use crate::command;
use crate::error::BoxResult;
use crate::manager::ManagerExt;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, RunEvent, Runtime};

pub use crate::collection::StoreCollectionBuilder as Builder;

/// Initializes the store plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  build(Builder::<R, DefaultMarker>::default())
}

pub(crate) fn build<R>(builder: Builder<R, DefaultMarker>) -> TauriPlugin<R>
where
  R: Runtime,
{
  tauri::plugin::Builder::new("tauri-store")
    .on_event(on_event)
    .setup(|app, _| setup(app, builder))
    .invoke_handler(tauri::generate_handler![
      command::allow_save,
      command::allow_sync,
      command::clear_autosave,
      command::deny_save,
      command::deny_sync,
      command::get_default_save_strategy,
      command::get_save_strategy,
      command::get_store_collection_path,
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

fn setup<R>(app: &AppHandle<R>, builder: Builder<R, DefaultMarker>) -> BoxResult<()>
where
  R: Runtime,
{
  builder.build(app, env!("CARGO_PKG_NAME"))?;
  Ok(())
}

fn on_event<R>(app: &AppHandle<R>, event: &RunEvent)
where
  R: Runtime,
{
  if let RunEvent::Exit = event {
    let _ = app.store_collection().on_exit();
  }
}
