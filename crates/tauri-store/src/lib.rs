#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod collection;
mod error;
mod event;
mod manager;
mod store;

pub use collection::{OnLoadFn, OnLoadResult, StoreCollection, StoreCollectionBuilder};
pub use error::{BoxResult, Error, Result};
pub use event::{STORE_UNLOADED_EVENT, STORE_UPDATED_EVENT};
pub use manager::ManagerExt;
pub use serde_json::Value as Json;
pub use store::{SaveStrategy, Store, StoreState, StoreStateExt, WatcherResult};
use tauri::{Manager, Runtime};

#[cfg(feature = "derive")]
pub use tauri_store_macros::{Collection, CollectionBuilder};

#[cfg(feature = "unstable-async")]
use {futures::future::BoxFuture, std::future::Future};

/// Calls a closure with a mutable reference to the store with the given id.
#[cfg(not(feature = "unstable-async"))]
pub fn with_store<R, M, F, T>(manager: &M, id: impl AsRef<str>, f: F) -> Result<T>
where
  R: Runtime,
  M: Manager<R> + ManagerExt<R>,
  F: FnOnce(&mut Store<R>) -> T,
{
  manager.store_collection().with_store(id, f)
}

/// Calls a closure with a mutable reference to the store with the given id.
#[cfg(feature = "unstable-async")]
pub async fn with_store<R, M, F, Fut, T>(manager: &M, id: impl AsRef<str>, f: F) -> Result<T>
where
  R: Runtime,
  M: Manager<R> + ManagerExt<R>,
  F: FnOnce(&mut Store<R>) -> Fut + Send,
  Fut: Future<Output = T> + Send,
  T: Send + 'static,
{
  manager.store_collection().with_store(id, f).await
}

/// Creates a boxed future wrapping the given value.
#[cfg(feature = "unstable-async")]
#[inline]
pub fn boxed<T>(t: T) -> BoxFuture<'static, T>
where
  T: Send + 'static,
{
  Box::pin(std::future::ready(t))
}
