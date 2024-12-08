use super::{ResourceTuple, Store};
use crate::error::Result;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Resource, ResourceId, Runtime};

#[cfg(feature = "unstable-async")]
use tokio::sync::Mutex as TokioMutex;

#[cfg(not(feature = "unstable-async"))]
use std::sync::Mutex as StdMutex;

pub(crate) struct StoreResource<R: Runtime> {
  #[cfg(feature = "unstable-async")]
  pub(crate) inner: TokioMutex<Store<R>>,
  #[cfg(not(feature = "unstable-async"))]
  pub(crate) inner: StdMutex<Store<R>>,
}

impl<R: Runtime> StoreResource<R> {
  fn new(store: Store<R>) -> Self {
    Self {
      #[cfg(feature = "unstable-async")]
      inner: TokioMutex::new(store),
      #[cfg(not(feature = "unstable-async"))]
      inner: StdMutex::new(store),
    }
  }

  pub(super) fn create(app: &AppHandle<R>, store: Store<R>) -> ResourceTuple<R> {
    let resource = Arc::new(Self::new(store));
    let rid = app
      .resources_table()
      .add_arc(Arc::clone(&resource));

    (rid, resource)
  }

  pub(crate) fn get(app: &AppHandle<R>, rid: ResourceId) -> Result<Arc<Self>> {
    app
      .resources_table()
      .get::<Self>(rid)
      .map_err(Into::into)
  }

  pub(crate) fn take(app: &AppHandle<R>, rid: ResourceId) -> Result<Arc<Self>> {
    app
      .resources_table()
      .take::<Self>(rid)
      .map_err(Into::into)
  }
}

impl<R: Runtime> Resource for StoreResource<R> {}
