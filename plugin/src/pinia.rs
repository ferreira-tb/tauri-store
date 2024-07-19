use crate::error::Result;
use crate::store::{State, Store};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Runtime};
use tracing::error;

#[cfg(feature = "ahash")]
use ahash::HashMap;
#[cfg(not(feature = "ahash"))]
use std::collections::HashMap;

pub struct Pinia<R: Runtime> {
  pub(crate) path: PathBuf,
  pub(crate) stores: Mutex<HashMap<String, Store<R>>>,
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

    f(stores.get_mut(id).unwrap())
  }

  /// Saves all the stores to the disk.
  pub fn save(&self) {
    let stores = self.stores.lock().unwrap();
    for store in stores.values() {
      if let Err(err) = store.save() {
        error!("failed to save store {}: {err}", store.id);
      }
    }
  }

  pub fn set(&self, app: &AppHandle<R>, id: impl AsRef<str>, state: State) -> Result<()> {
    self.with_store(app, id, |store| store.set(state, None))
  }
}
