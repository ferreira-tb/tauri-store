use crate::pinia::Pinia;
use tauri::{AppHandle, Manager, Runtime, State, WebviewWindow, Window};

pub trait ManagerExt<R: Runtime>: Manager<R> {
  /// Returns a handle to the pinia plugin.
  ///
  /// # Panics
  ///
  /// Panics if the internal store collection is not in the [resources table](tauri::ResourceTable).
  ///
  /// This likely indicates that the method was called before the plugin was properly initialized.
  fn pinia(&self) -> State<Pinia<R>> {
    self.state::<Pinia<R>>()
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
