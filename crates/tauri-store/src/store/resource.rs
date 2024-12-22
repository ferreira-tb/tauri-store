use super::{ResourceTuple, Store};
use crate::error::Result;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, Resource, ResourceId, Runtime};

pub(crate) struct StoreResource<R: Runtime> {
  pub(crate) inner: Mutex<Store<R>>,
}

impl<R: Runtime> StoreResource<R> {
  fn new(store: Store<R>) -> Self {
    Self { inner: Mutex::new(store) }
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

// Using StoreResource directly avoids the StoreCollection trying
// to load the store if it isn't in the resources table already.
impl<R: Runtime> StoreResource<R> {
  pub(crate) fn locked<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&mut Store<R>) -> T,
  {
    f(&mut *self.inner.lock().unwrap())
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
}

impl<R: Runtime> Resource for StoreResource<R> {}
