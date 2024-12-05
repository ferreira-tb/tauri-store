use crate::pinia::Pinia;
use tauri::{AppHandle, Manager, Runtime, State, WebviewWindow, Window};

pub trait ManagerExt<R: Runtime>: Manager<R> {
  fn pinia(&self) -> State<Pinia<R>> {
    self.state::<Pinia<R>>()
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
