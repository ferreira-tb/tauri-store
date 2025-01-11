use crate::collection::{StoreCollection, RESOURCE_ID};
use crate::error::Result;
use crate::store::Store;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow, Window};

/// Extension for the [`Manager`] trait providing access to the store collection.
///
/// [`Manager`]: https://docs.rs/tauri/latest/tauri/trait.Manager.html
pub trait ManagerExt<R: Runtime>: Manager<R> {
  /// Returns a handle to the store collection.
  ///
  /// # Panics
  ///
  /// Panics if the [store collection](crate::collection::StoreCollection) is not in the [resources table].
  ///
  /// This likely indicates that the method was called before the plugin was properly initialized.
  ///
  /// [resources table]: https://docs.rs/tauri/latest/tauri/struct.ResourceTable.html
  fn store_collection(&self) -> Arc<StoreCollection<R>> {
    let rid = RESOURCE_ID
      .get()
      .expect("missing store collection resource id");

    self
      .resources_table()
      .get::<StoreCollection<R>>(*rid)
      .expect("store collection is not in the resources table")
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> T,
  {
    self.store_collection().with_store(id, f)
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
