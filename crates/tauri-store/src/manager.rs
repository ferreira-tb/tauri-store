use crate::collection::StoreCollection;
use crate::error::Result;
use crate::store::Store;
use crate::{CollectionMarker, DefaultMarker};
use tauri::{Manager, Runtime, State};

/// Extension for the [`Manager`] trait providing access to the store collection.
///
/// [`Manager`]: https://docs.rs/tauri/latest/tauri/trait.Manager.html
pub trait ManagerExt<R: Runtime>: Manager<R> {
  /// Returns a handle to the default store collection.
  ///
  /// # Panics
  ///
  /// Panics if the [store collection] is not yet being managed by Tauri.
  ///
  /// This likely indicates it was called before the plugin was properly initialized.
  ///
  /// [store collection]: https://docs.rs/tauri-store/latest/tauri_store/struct.StoreCollection.html
  fn store_collection(&self) -> State<'_, StoreCollection<R, DefaultMarker>> {
    self.store_collection_with_marker::<DefaultMarker>()
  }

  /// Returns a handle to the store collection for the specified marker.
  ///
  /// # Panics
  ///
  /// Panics if the [store collection] is not yet being managed by Tauri.
  ///
  /// This likely indicates it was called before the plugin was properly initialized.
  ///
  /// [store collection]: https://docs.rs/tauri-store/latest/tauri_store/struct.StoreCollection.html
  fn store_collection_with_marker<C>(&self) -> State<'_, StoreCollection<R, C>>
  where
    C: CollectionMarker,
  {
    self.app_handle().state::<StoreCollection<R, C>>()
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R, DefaultMarker>) -> T,
  {
    self.store_collection().with_store(id, f)
  }
}

impl<R: Runtime, T: Manager<R>> ManagerExt<R> for T {}
