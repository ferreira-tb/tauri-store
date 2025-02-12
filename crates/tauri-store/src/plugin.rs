use crate::command;
use crate::manager::ManagerExt;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, RunEvent, Runtime};

pub use crate::collection::StoreCollectionBuilder as Builder;

/// Initializes the store plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  create(Builder::default())
}

pub(crate) fn create<R: Runtime>(builder: Builder<R>) -> TauriPlugin<R> {
  tauri::plugin::Builder::new("tauri-store")
    .on_event(on_event)
    .setup(|app, _| {
      let _ = builder.build(app);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      command::clear_autosave,
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

fn on_event<R: Runtime>(app: &AppHandle<R>, event: &RunEvent) {
  if let RunEvent::Exit = event {
    let _ = app.store_collection().on_exit();
  }
}
