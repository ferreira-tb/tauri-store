use crate::error::Result;
use crate::store::Store;
use std::path::{Path, PathBuf};
use tauri::{Manager, Runtime};

#[cfg(not(feature = "async-pinia"))]
use std::sync::Mutex;
#[cfg(feature = "async-pinia")]
use {crate::BoxFuture, tauri::async_runtime::Mutex};

#[cfg(feature = "ahash")]
use ahash::{HashMap, HashSet};
#[cfg(not(feature = "ahash"))]
use std::collections::{HashMap, HashSet};

pub struct Pinia<R: Runtime> {
  pub(crate) path: PathBuf,
  pub(crate) stores: Mutex<HashMap<String, Store<R>>>,
  pub(crate) sync_denylist: HashSet<String>,
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

  /// Saves all the stores to the disk.
  #[cfg(not(feature = "async-pinia"))]
  pub fn save(&self) {
    let stores = self.stores.lock().unwrap();
    for store in stores.values() {
      #[cfg_attr(not(feature = "tracing"), allow(unused_variables))]
      if let Err(err) = store.save() {
        #[cfg(feature = "tracing")]
        tracing::error!("failed to save store {}: {}", store.id, err);
      }
    }
  }

  /// Saves all the stores to the disk.
  #[cfg(feature = "async-pinia")]
  pub async fn save(&self) {
    let stores = self.stores.lock().await;
    for store in stores.values() {
      #[cfg_attr(not(feature = "tracing"), allow(unused_variables))]
      if let Err(err) = store.save().await {
        #[cfg(feature = "tracing")]
        tracing::error!("failed to save store {}: {}", store.id, err);
      }
    }
  }
}
