use super::StoreCollection;
use crate::error::Result;
use crate::io_err;
use crate::meta::Meta;
use crate::store::{append_filename, Store, StoreResource};
use itertools::Itertools;
use std::path::{Path, PathBuf};
use std::{fs, mem};
use tauri::Runtime;

impl<R: Runtime> StoreCollection<R> {
  /// Directory where the stores are saved.
  pub fn path(&self) -> PathBuf {
    self.path.lock().unwrap().clone()
  }

  /// Sets the directory where the stores are saved.
  /// This will move all *currently active* stores to the new directory.
  pub fn set_path(&self, path: impl AsRef<Path>) -> Result<()> {
    let new = path.as_ref().to_path_buf();
    if new == *self.path.lock().unwrap() {
      return Ok(());
    }

    fs::create_dir_all(&new)?;

    let resources = self
      .rids()
      .into_iter()
      .map(|rid| StoreResource::get(&self.app, rid))
      .process_results(|res| res.collect_vec())
      .unwrap_or_default();

    if resources.is_empty() {
      *self.path.lock().unwrap() = new;
      return Ok(());
    }

    // Locking all the stores first ensures that none of them will attempt to lock the path.
    // By itself, this should not cause a deadlock, as the stores don't depend on each other.
    {
      let stores = resources
        .iter()
        .map(|resource| resource.inner.lock())
        .process_results(|res| res.collect_vec())
        .unwrap_or_default();

      let mut lock = self.path.lock().unwrap();
      let from = mem::replace(&mut *lock, new);
      let to = &*lock;

      for store in stores {
        move_store(&*store, &from, to)?;
      }
    }

    // We need to ensure that the path lock is released before calling this.
    Meta::write(self)?;

    Ok(())
  }
}

fn move_store<R>(store: &Store<R>, from: &Path, to: &Path) -> Result<()>
where
  R: Runtime,
{
  // Calling `Store::path` would be a deadlock!
  // We need to manually append the filename to the path.
  let current = append_filename(from, &store.id);
  let new = append_filename(to, &store.id);

  if new.try_exists()? {
    let path = new.display();
    return io_err!(AlreadyExists, "file already exists: {path}");
  }

  fs::copy(&current, &new)?;
  fs::remove_file(&current)?;

  Ok(())
}
