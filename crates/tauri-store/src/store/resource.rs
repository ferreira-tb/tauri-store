use super::{ResourceTuple, Store};
use crate::collection::CollectionMarker;
use crate::error::Result;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, Resource, ResourceId, Runtime};

pub(crate) struct StoreResource<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
  pub(crate) inner: Mutex<Store<R, C>>,
}

impl<R, C> StoreResource<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
  fn new(store: Store<R, C>) -> Self {
    Self { inner: Mutex::new(store) }
  }

  pub(super) fn create(app: &AppHandle<R>, store: Store<R, C>) -> ResourceTuple<R, C> {
    let resource = Arc::new(Self::new(store));
    let rid = app
      .resources_table()
      .add_arc(Arc::clone(&resource));

    (rid, resource)
  }

  // Using the StoreResource directly avoids the StoreCollection trying
  // to load the store if it isn't in the resources table already.
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

  pub(crate) fn save(app: &AppHandle<R>, rid: ResourceId) -> Result<()> {
    Self::get(app, rid)?.locked(|store| store.save())
  }

  pub(crate) fn save_now(app: &AppHandle<R>, rid: ResourceId) -> Result<()> {
    Self::get(app, rid)?.locked(|store| {
      store.abort_pending_save();
      store.save_now()
    })
  }

  pub(crate) fn locked<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&mut Store<R, C>) -> T,
  {
    f(&mut *self.inner.lock().unwrap())
  }
}

impl<R, C> Resource for StoreResource<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
}
