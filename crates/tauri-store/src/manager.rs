use crate::collection::{StoreCollection, RESOURCE_ID};
use std::sync::Arc;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow, Window};

/// Extension for the [`Manager`](tauri::Manager) trait providing access to the store collection.
pub trait ManagerExt<R: Runtime>: Manager<R> {
  /// Returns a handle to the store collection.
  ///
  /// # Panics
  ///
  /// Panics if the store collection is not in the [resources table](tauri::ResourceTable).
  ///
  /// This likely indicates that the method was called before the plugin was properly initialized.
  fn store_collection(&self) -> Arc<StoreCollection<R>> {
    let rid = RESOURCE_ID
      .get()
      .expect("missing store collection resource id");

    self
      .resources_table()
      .get::<StoreCollection<R>>(*rid)
      .expect("store collection is not in the resources table")
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
