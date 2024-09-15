use crate::error::Result;
use crate::pinia::Pinia;
use crate::store::Store;
#[cfg(feature = "async-pinia")]
use crate::{BoxFuture, FutureExt};
use tauri::{AppHandle, Manager, Runtime, WebviewWindow, Window};

pub trait ManagerExt<R: Runtime>: Manager<R> {
  fn pinia(&self) -> tauri::State<Pinia<R>> {
    self.state::<Pinia<R>>()
  }

  #[cfg(not(feature = "async-pinia"))]
  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> Result<T>,
  {
    self.pinia().with_store(self.app_handle(), id, f)
  }

  #[cfg(feature = "async-pinia")]
  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> BoxFuture<Result<T>>
  where
    F: FnOnce(&mut Store<R>) -> BoxFuture<Result<T>> + Send + 'static,
    T: Send + 'static,
  {
    let id = id.as_ref().to_owned();
    let app = self.app_handle().clone();
    async move { app.pinia().with_store(&app, id, f).await }.boxed()
  }

  #[cfg(not(feature = "async-pinia"))]
  fn save_store(&self, id: impl AsRef<str>) -> Result<()> {
    self.with_store(id, |store| store.save())
  }

  #[cfg(feature = "async-pinia")]
  async fn save_store(&self, id: impl AsRef<str>) -> Result<()> {
    self
      .with_store(id, |store| store.save().boxed())
      .await
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
