use crate::__SNAKE_PLUGIN_TITLE__::__PASCAL_PLUGIN_TITLE__;
use tauri::{AppHandle, Manager, Runtime, State, WebviewWindow, Window};
use tauri_store::{Result, Store};

/// Extension for the [`Manager`] trait providing access to the __PASCAL_PLUGIN_TITLE__ plugin.
///
/// [`Manager`]: https://docs.rs/tauri/latest/tauri/trait.Manager.html
pub trait ManagerExt<R: Runtime>: Manager<R> {
  /// Returns a handle to the __PASCAL_PLUGIN_TITLE__ plugin.
  ///
  /// # Panics
  ///
  /// Panics if the internal [store collection] is not in the [resources table].
  ///
  /// This likely indicates that the method was called before the plugin was properly initialized.
  ///
  /// [store collection]: https://docs.rs/tauri-store/latest/tauri_store/struct.StoreCollection.html
  /// [resources table]: https://docs.rs/tauri/latest/tauri/struct.ResourceTable.html
  fn __SNAKE_PLUGIN_TITLE__(&self) -> State<__PASCAL_PLUGIN_TITLE__<R>> {
    self
      .app_handle()
      .state::<__PASCAL_PLUGIN_TITLE__<R>>()
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> T,
  {
    self.__SNAKE_PLUGIN_TITLE__().with_store(id, f)
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
