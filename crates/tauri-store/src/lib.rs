#![cfg_attr(docsrs, feature(doc_cfg))]

mod collection;
mod error;
mod event;
mod listener;
mod manager;
mod store;

pub use collection::{StoreCollection, StoreCollectionBuilder};
pub use error::{BoxResult, Error, Result};
pub use event::{STORE_UNLOADED_EVENT, STORE_UPDATED_EVENT};
pub use listener::WatcherResult;
pub use manager::ManagerExt;
pub use serde_json::Value as Json;
pub use store::{Store, StoreState};
use tauri::{Manager, Runtime};

#[cfg(feature = "derive")]
pub use tauri_store_macros::Collection;

#[cfg(feature = "unstable-async")]
use {std::future::Future, std::pin::Pin};

#[cfg(feature = "unstable-async")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-async")))]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[cfg(not(feature = "unstable-async"))]
pub fn with_store<R, M, F, T>(manager: &M, id: impl AsRef<str>, f: F) -> Result<T>
where
  R: Runtime,
  M: Manager<R> + ManagerExt<R>,
  F: FnOnce(&mut Store<R>) -> Result<T>,
{
  manager.store_collection().with_store(id, f)
}

#[cfg(feature = "unstable-async")]
pub async fn with_store<R, M, F, T>(manager: &M, id: impl AsRef<str>, f: F) -> Result<T>
where
  R: Runtime,
  M: Manager<R> + ManagerExt<R>,
  F: FnOnce(&mut Store<R>) -> BoxFuture<Result<T>> + Send + 'static,
  T: Send + 'static,
{
  manager.store_collection().with_store(id, f).await
}

#[cfg(feature = "unstable-async")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-async")))]
pub trait FutureExt: Future {
  /// Wrap the future in a Box, pinning it.
  fn boxed<'a>(self) -> BoxFuture<'a, Self::Output>
  where
    Self: Sized + Send + 'a,
  {
    Box::pin(self)
  }

  fn boxed_ok<'a>(self) -> BoxFuture<'a, Result<Self::Output>>
  where
    Self: Sized + Send + 'a,
  {
    Box::pin(async move { Ok(self.await) })
  }
}

#[cfg(feature = "unstable-async")]
impl<T> FutureExt for T where T: ?Sized + Future {}
