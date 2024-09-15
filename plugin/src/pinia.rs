use crate::error::Result;
use crate::io_err;
use crate::store::{Store, StoreState};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::path::{Path, PathBuf};
use tauri::{Manager, Runtime};

#[cfg(feature = "async-pinia")]
use {crate::BoxFuture, std::time::Duration, tokio::task::AbortHandle};

#[cfg(feature = "ahash")]
use ahash::{HashMap, HashSet};
#[cfg(not(feature = "ahash"))]
use std::collections::{HashMap, HashSet};

pub struct Pinia<R: Runtime> {
  pub(crate) path: PathBuf,
  pub(crate) sync_denylist: HashSet<String>,

  #[cfg(not(feature = "async-pinia"))]
  pub(crate) stores: std::sync::Mutex<HashMap<String, Store<R>>>,
  #[cfg(feature = "async-pinia")]
  pub(crate) stores: tokio::sync::Mutex<HashMap<String, Store<R>>>,

  #[cfg(feature = "async-pinia")]
  pub(crate) autosave: std::sync::Mutex<Option<AbortHandle>>,
}

macro_rules! save_one {
  ($store:expr, $save_call:expr) => {
    #[cfg_attr(not(feature = "tracing"), allow(unused_variables))]
    if let Err(err) = $save_call {
      #[cfg(feature = "tracing")]
      tracing::error!("failed to save store {}: {}", $store.id, err);
    }
  };
}

impl<R: Runtime> Pinia<R> {
  pub fn path(&self) -> &Path {
    &self.path
  }

  #[cfg(not(feature = "async-pinia"))]
  pub fn with_store<M, F, T>(&self, manager: &M, id: impl AsRef<str>, f: F) -> Result<T>
  where
    M: Manager<R>,
    F: FnOnce(&mut Store<R>) -> Result<T>,
  {
    let id = id.as_ref();
    let mut stores = self.stores.lock().unwrap();
    if !stores.contains_key(id) {
      let app = manager.app_handle();
      let store = Store::load(app.clone(), id)?;
      stores.insert(id.to_owned(), store);
    }

    f(stores.get_mut(id).expect("store should exist"))
  }

  #[cfg(feature = "async-pinia")]
  pub fn with_store<M, F, T>(&self, manager: &M, id: impl AsRef<str>, f: F) -> BoxFuture<Result<T>>
  where
    M: Manager<R>,
    F: FnOnce(&mut Store<R>) -> BoxFuture<Result<T>> + Send + 'static,
    T: Send + 'static,
  {
    let id = id.as_ref().to_owned();
    let app = manager.app_handle().clone();
    Box::pin(async move {
      let mut stores = self.stores.lock().await;
      if !stores.contains_key(&id) {
        let store = Store::load(app, &id).unwrap();
        stores.insert(id.clone(), store);
      }

      f(stores.get_mut(&id).expect("store should exist")).await
    })
  }

  #[cfg(not(feature = "async-pinia"))]
  pub fn ids(&self) -> Vec<String> {
    let stores = self.stores.lock().unwrap();
    stores.keys().cloned().collect()
  }

  #[cfg(feature = "async-pinia")]
  pub async fn ids(&self) -> Vec<String> {
    let stores = self.stores.lock().await;
    stores.keys().cloned().collect()
  }

  /// Gets a clone of the store state if it exists.
  ///
  /// **WARNING:** Changes to the returned state will not be reflected in the store.
  #[cfg(not(feature = "async-pinia"))]
  pub fn store_state(&self, store_id: impl AsRef<str>) -> Option<StoreState> {
    let stores = self.stores.lock().unwrap();
    stores.get(store_id.as_ref()).map(Store::state)
  }

  /// Gets a clone of the store state if it exists.
  ///
  /// **WARNING:** Changes to the returned state will not be reflected in the store.
  #[cfg(feature = "async-pinia")]
  pub async fn store_state(&self, store_id: impl AsRef<str>) -> Option<StoreState> {
    let stores = self.stores.lock().await;
    stores.get(store_id.as_ref()).map(Store::state)
  }

  /// Gets the store state if it exists, then tries to deserialize it as an instance of type `T`.
  #[cfg(not(feature = "async-pinia"))]
  pub fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let stores = self.stores.lock().unwrap();
    let Some(store) = stores.get(store_id.as_ref()) else {
      return io_err!(NotFound, "store not found: {}", store_id.as_ref());
    };

    let state = json!(store.state());
    serde_json::from_value(state).map_err(Into::into)
  }

  /// Gets the store state if it exists, then tries to deserialize it as an instance of type `T`.
  #[cfg(feature = "async-pinia")]
  pub async fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let stores = self.stores.lock().await;
    let Some(store) = stores.get(store_id.as_ref()) else {
      return io_err!(NotFound, "store not found: {}", store_id.as_ref());
    };

    let state = json!(store.state());
    serde_json::from_value(state).map_err(Into::into)
  }

  #[cfg(not(feature = "async-pinia"))]
  pub(crate) fn unload_store(&self, id: &str) {
    let mut stores = self.stores.lock().unwrap();
    if let Some(store) = stores.remove(id) {
      drop(stores);
      save_one!(store, store.save());
    }
  }

  #[cfg(feature = "async-pinia")]
  pub(crate) async fn unload_store(&self, id: &str) {
    let mut stores = self.stores.lock().await;
    if let Some(store) = stores.remove(id) {
      drop(stores);
      save_one!(store, store.save().await);
    }
  }

  /// Saves all the stores to the disk.
  #[cfg(not(feature = "async-pinia"))]
  pub fn save_all(&self) {
    let stores = self.stores.lock().unwrap();
    for store in stores.values() {
      save_one!(store, store.save());
    }
  }

  /// Saves all the stores to the disk.
  #[cfg(feature = "async-pinia")]
  pub async fn save_all(&self) {
    let stores = self.stores.lock().await;
    for store in stores.values() {
      save_one!(store, store.save().await);
    }
  }

  /// Saves some stores to the disk.
  #[cfg(not(feature = "async-pinia"))]
  pub fn save_some(&self, ids: &[impl AsRef<str>]) {
    let stores = self.stores.lock().unwrap();
    for id in ids {
      if let Some(store) = stores.get(id.as_ref()) {
        save_one!(store, store.save());
      }
    }
  }

  /// Saves some stores to the disk.
  #[cfg(feature = "async-pinia")]
  pub async fn save_some(&self, ids: &[impl AsRef<str>]) {
    let stores = self.stores.lock().await;
    for id in ids {
      if let Some(store) = stores.get(id.as_ref()) {
        save_one!(store, store.save().await);
      }
    }
  }

  /// Saves the stores periodically.
  #[cfg(feature = "async-pinia")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async-pinia")))]
  pub fn set_autosave<M>(&self, manager: &M, duration: Duration)
  where
    M: Manager<R>,
  {
    use crate::ManagerExt;
    use tauri::async_runtime::{self, RuntimeHandle};
    use tokio::time::{self, MissedTickBehavior};

    self.clear_autosave();

    let app = manager.app_handle().clone();
    let RuntimeHandle::Tokio(runtime) = async_runtime::handle();
    let task = runtime.spawn(async move {
      let mut interval = time::interval(duration);
      interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
      loop {
        interval.tick().await;
        app.pinia().save_all().await;
      }
    });

    let mut guard = self.autosave.lock().unwrap();
    *guard = Some(task.abort_handle());
  }

  #[cfg(feature = "async-pinia")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async-pinia")))]
  pub fn clear_autosave(&self) {
    let mut guard = self.autosave.lock().unwrap();
    if let Some(autosave) = guard.take() {
      drop(guard);
      autosave.abort();
    }
  }
}
