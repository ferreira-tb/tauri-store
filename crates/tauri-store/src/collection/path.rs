use super::StoreCollection;
use crate::error::Result;
use crate::io_err;
use crate::store::{append_filename as append, Store, StoreResource};
use itertools::Itertools;
use std::path::Path;
use std::{fs, mem};
use tauri::Runtime;

pub(super) fn set_path<R>(collection: &StoreCollection<R>, path: impl AsRef<Path>) -> Result<()>
where
  R: Runtime,
{
  let new = path.as_ref().to_path_buf();
  if new == *collection.path.lock().unwrap() {
    return Ok(());
  }

  fs::create_dir_all(&new)?;

  let resources = collection
    .rids()
    .into_iter()
    .map(|rid| StoreResource::get(&collection.app, rid))
    .process_results(|it| it.collect_vec())
    .unwrap_or_default();

  if resources.is_empty() {
    *collection.path.lock().unwrap() = new;
    return Ok(());
  }

  // Locking all the stores first ensures that none of them will attempt to lock the path.
  // By itself, this should not cause a deadlock, as the stores don't depend on each other.
  let stores = resources
    .iter()
    .map(|resource| resource.inner.lock())
    .process_results(|it| it.collect_vec())
    .unwrap_or_default();

  let mut lock = collection.path.lock().unwrap();
  let from = mem::replace(&mut *lock, new);
  let to = &*lock;

  for store in stores {
    move_store(&*store, &from, to)?;
  }

  Ok(())
}

fn move_store<R>(store: &Store<R>, from: &Path, to: &Path) -> Result<()>
where
  R: Runtime,
{
  // Calling `Store::path` would be a deadlock!
  // We need to manually append the filename to the path.
  let id = store.id();
  let current = append(from, id);
  let new = append(to, id);

  if new.try_exists()? {
    let path = new.display();
    return io_err!(AlreadyExists, "file already exists: {path}");
  }

  fs::copy(&current, &new)?;
  fs::remove_file(&current)?;

  Ok(())
}
