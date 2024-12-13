use std::future::Future;
use tauri::async_runtime::{self, RuntimeHandle, TokioHandle};
use tauri::{AppHandle, Manager, Runtime, WebviewWindow, Window};
use tokio::task::AbortHandle;

pub trait ManagerExt<R: Runtime>: Manager<R> {
  fn tokio_handle(&self) -> TokioHandle {
    let RuntimeHandle::Tokio(handle) = async_runtime::handle();
    handle
  }

  fn spawn<F, Fut>(&self, f: F) -> AbortHandle
  where
    F: FnOnce(AppHandle<R>) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
  {
    self
      .tokio_handle()
      .spawn(f(self.app_handle().clone()))
      .abort_handle()
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
