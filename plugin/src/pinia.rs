use crate::error::Result;
use crate::store::Store;
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
  pub(crate) fn unload_store(&self, id: &str) {
    let mut stores = self.stores.lock().unwrap();
    if let Some(store) = stores.remove(id) {
      drop(stores);
      save_one!(store, store.save());

      #[cfg(feature = "tracing")]
      tracing::info!("store {id} unloaded");
    }
  }

  #[cfg(feature = "async-pinia")]
  pub(crate) async fn unload_store(&self, id: &str) {
    let mut stores = self.stores.lock().await;
    if let Some(store) = stores.remove(id) {
      drop(stores);
      save_one!(store, store.save().await);

      #[cfg(feature = "tracing")]
      tracing::info!("store {id} unloaded");
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
      #[cfg(feature = "tracing")]
      tracing::info!("autosaving enabled: {:?}", duration);

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
  pub fn clear_autosave(&self) {
    let mut guard = self.autosave.lock().unwrap();
    if let Some(autosave) = guard.take() {
      autosave.abort();

      #[cfg(feature = "tracing")]
      tracing::info!("autosave cleared");
    }
  }
}
