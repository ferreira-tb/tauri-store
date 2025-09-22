use crate::collection::{DefaultMarker, Handle, StoreCollectionBuilder};
use crate::command;
use crate::error::BoxResult;
use crate::manager::ManagerExt;
use serde::de::DeserializeOwned;
use tauri::plugin::{PluginApi, TauriPlugin};
use tauri::{AppHandle, RunEvent, Runtime};

pub use crate::collection::StoreCollectionBuilder as Builder;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_tauri_store);

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
    .setup(|app, api| setup(app, api, builder))
    .invoke_handler(tauri::generate_handler![
      command::allow_save,
      command::allow_sync,
      command::clear_autosave,
      command::deny_save,
      command::deny_sync,
      command::destroy,
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
      command::set_store_options,
      command::unload
    ])
    .build()
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub(super) fn setup<R, D>(
  app: &AppHandle<R>,
  _api: PluginApi<R, D>,
  builder: StoreCollectionBuilder<R, DefaultMarker>,
) -> BoxResult<()>
where
  R: Runtime,
  D: DeserializeOwned,
{
  let handle = Handle::new(app.clone());
  builder.build(handle, env!("CARGO_PKG_NAME"))?;
  Ok(())
}

#[cfg(any(target_os = "android", target_os = "ios"))]
pub(super) fn setup<R, D>(
  _app: &AppHandle<R>,
  api: PluginApi<R, D>,
  builder: StoreCollectionBuilder<R, DefaultMarker>,
) -> BoxResult<()>
where
  R: Runtime,
  D: DeserializeOwned,
{
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("", "TauriStorePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_tauri_store)?;

  builder.build(Handle::new(handle), env!("CARGO_PKG_NAME"))?;

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
