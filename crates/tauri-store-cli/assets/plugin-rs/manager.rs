use crate::PLUGIN_NAME::PASCAL_PLUGIN_NAME;
use tauri::{AppHandle, Manager, Runtime, State, WebviewWindow, Window};

/// Extension for the [`Manager`](https://docs.rs/tauri/latest/tauri/trait.Manager.html)
/// trait providing access to the PASCAL_PLUGIN_NAME plugin.
pub trait ManagerExt<R: Runtime>: Manager<R> {
  /// Returns a handle to the PASCAL_PLUGIN_NAME plugin.
  ///
  /// # Panics
  ///
  /// Panics if the internal [store collection](https://tb.dev.br/tauri-store/rust-docs/tauri_store/struct.StoreCollection.html)
  /// is not in the [resources table](https://docs.rs/tauri/latest/tauri/struct.ResourceTable.html).
  ///
  /// This likely indicates that the method was called before the plugin was properly initialized.
  fn PLUGIN_NAME(&self) -> State<PASCAL_PLUGIN_NAME<R>> {
    self.state::<PASCAL_PLUGIN_NAME<R>>()
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
