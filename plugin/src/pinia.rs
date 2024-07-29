use crate::error::Result;
use crate::store::Store;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Runtime};

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

  pub fn with_store<F, T>(&self, app: &AppHandle<R>, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> Result<T>,
  {
    let id = id.as_ref();
    let mut stores = self.stores.lock().unwrap();
    if !stores.contains_key(id) {
      let store = Store::load(app.clone(), id)?;
      stores.insert(id.to_owned(), store);
    }

    f(stores.get_mut(id).expect("store should exist"))
  }

  /// Saves all the stores to the disk.
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
}
