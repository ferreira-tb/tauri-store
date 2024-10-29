#![cfg_attr(docsrs, feature(doc_cfg))]

mod collection;
mod error;
mod event;
mod manager;
mod state;
mod store;
mod watch;

pub use collection::{OnLoadFn, OnLoadResult, StoreCollection, StoreCollectionBuilder};
pub use error::{BoxResult, Error, Result};
pub use event::{STORE_UNLOADED_EVENT, STORE_UPDATED_EVENT};
pub use manager::ManagerExt;
pub use serde_json::Value as Json;
pub use state::{StoreState, StoreStateExt};
pub use store::Store;
use tauri::{Manager, Runtime};
pub use watch::WatcherResult;

#[cfg(feature = "derive")]
pub use tauri_store_macros::Collection;

#[cfg(feature = "unstable-async")]
pub use futures::future::{BoxFuture, FutureExt};

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
#[macro_export]
macro_rules! boxed {
  { $($t:tt)* } => {{
      Box::pin(async move { $($t)* })
  }};
}

#[cfg(feature = "unstable-async")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-async")))]
#[macro_export]
macro_rules! boxed_ok {
  { $($t:tt)* } => {{
      Box::pin(async move { Ok($($t)*) })
  }};
}
