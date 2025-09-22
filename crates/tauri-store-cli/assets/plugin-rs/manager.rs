use crate::__SNAKE_PLUGIN_TITLE__::{__PASCAL_PLUGIN_TITLE__, __PASCAL_PLUGIN_TITLE__Marker};
use tauri::{Manager, Runtime};
use tauri_store::{ManagerExt as _, Result, Store};

/// Extension for the [`Manager`] trait providing access to the __PASCAL_PLUGIN_TITLE__ plugin.
///
/// [`Manager`]: https://docs.rs/tauri/latest/tauri/trait.Manager.html
pub trait ManagerExt<R: Runtime>: Manager<R> {
  /// Returns a handle to the __PASCAL_PLUGIN_TITLE__ plugin.
  ///
  /// # Panics
  ///
  /// Panics if the internal [store collection] is not yet being managed by Tauri.
  ///
  /// This likely indicates that it was called before the plugin was properly initialized.
  ///
  /// [store collection]: https://docs.rs/tauri-store/latest/tauri_store/struct.StoreCollection.html
  fn __SNAKE_PLUGIN_TITLE__(&self) -> __PASCAL_PLUGIN_TITLE__<'_, R> {
    __PASCAL_PLUGIN_TITLE__(
      self
        .app_handle()
        .store_collection_with_marker::<__PASCAL_PLUGIN_TITLE__Marker>(),
    )
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R, __PASCAL_PLUGIN_TITLE__Marker>) -> T,
  {
    self
      .app_handle()
      .store_collection_with_marker::<__PASCAL_PLUGIN_TITLE__Marker>()
      .with_store(id, f)
  }
}

impl<R: Runtime, T: Manager<R>> ManagerExt<R> for T {}
