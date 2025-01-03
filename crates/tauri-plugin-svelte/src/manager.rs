use crate::svelte::Svelte;
use tauri::{AppHandle, Manager, Runtime, State, WebviewWindow, Window};

/// Extension for the [`Manager`](tauri::Manager) trait providing access to the Svelte plugin.
pub trait ManagerExt<R: Runtime>: Manager<R> {
  /// Returns a handle to the Svelte plugin.
  ///
  /// # Panics
  ///
  /// Panics if the internal [store collection](tauri_store::StoreCollection) is not in the [resources table](tauri::ResourceTable).
  ///
  /// This likely indicates that the method was called before the plugin was properly initialized.
  fn svelte(&self) -> State<Svelte<R>> {
    self.state::<Svelte<R>>()
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
