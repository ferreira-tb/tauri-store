use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_collection(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_collection {
      use super::#name;
      use serde::de::DeserializeOwned;
      use std::path::Path;
      use std::sync::Arc;
      use std::time::Duration;
      use tauri::{AppHandle, Runtime};
      use tauri_store::{Json, Result, SaveStrategy, Store, StoreState};

      impl<R: Runtime> #name<R> {
        /// Stops the autosave.
        pub fn clear_autosave(&self) {
          self.0.clear_autosave()
        }

        /// Default save strategy for the stores.
        /// This can be overridden on a per-store basis.
        pub fn default_save_strategy(&self) -> SaveStrategy {
          self.0.default_save_strategy()
        }

        /// Gets a value from a store.
        pub fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<Json> {
          self.0.get(store_id, key)
        }

        /// Lists all the store ids.
        pub fn ids(&self) -> Vec<String> {
          self.0.ids()
        }

        /// Patches a store state.
        pub fn patch(&self, store_id: impl AsRef<str>, state: StoreState) -> Result<()> {
          self.0.patch(store_id, state)
        }

        /// Directory where the stores are saved.
        pub fn path(&self) -> &Path {
          &self.0.path()
        }

        /// Saves a store to the disk.
        pub fn save(&self, id: impl AsRef<str>) -> Result<()> {
          self.0.save(id)
        }

        /// Saves all the stores to the disk.
        pub fn save_all(&self) -> Result<()> {
          self.0.save_all()
        }

        /// Saves all the stores to the disk immediately, ignoring the save strategy.
        pub fn save_all_now(&self) -> Result<()> {
          self.0.save_all_now()
        }

        /// Saves a store to the disk immediately, ignoring the save strategy.
        pub fn save_now(&self, id: impl AsRef<str>) -> Result<()> {
          self.0.save_now(id)
        }

        /// Saves some stores to the disk.
        pub fn save_some(&self, ids: &[impl AsRef<str>]) -> Result<()> {
          self.0.save_some(ids)
        }

        /// Saves some stores to the disk immediately, ignoring the save strategy.
        pub fn save_some_now(&self, ids: &[impl AsRef<str>]) -> Result<()> {
          self.0.save_some_now(ids)
        }

        /// Sets a key-value pair in a store.
        pub fn set(&self, id: impl AsRef<str>, key: impl AsRef<str>, value: Json) -> Result<()> {
          self.0.set(id, key, value)
        }

        /// Saves the stores periodically.
        pub fn set_autosave(&self, duration: Duration) {
          self.0.set_autosave(duration)
        }

        /// Gets a clone of the store state if it exists.
        pub fn store_state(&self, store_id: impl AsRef<str>) -> Result<StoreState> {
          self.0.store_state(store_id)
        }

        /// Gets a value from a store and tries to parse it as an instance of type `T`.
        pub fn try_get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Result<T>
        where
          T: DeserializeOwned,
        {
          self.0.try_get(store_id, key)
        }

        /// Gets the store state if it exists, then tries to parse it as an instance of type `T`.
        pub fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> Result<T>
        where
          T: DeserializeOwned,
        {
          self.0.try_store_state(store_id)
        }

        pub(crate) fn unload_store(&self, id: &str) -> Result<()> {
          self.0.unload_store(id)
        }

        /// Removes a watcher from a store.
        pub fn unwatch(&self, store_id: impl AsRef<str>, listener_id: u32) -> Result<bool> {
          self.0.unwatch(store_id, listener_id)
        }

        /// Watches a store for changes.
        pub fn watch<F>(&self, store_id: impl AsRef<str>, f: F) -> Result<u32>
        where
          F: Fn(AppHandle<R>) -> Result<()> + Send + Sync + 'static,
        {
          self.0.watch(store_id, f)
        }

        /// Calls a closure with a mutable reference to the store with the given id.
        pub fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
        where
          F: FnOnce(&mut Store<R>) -> T,
        {
          self.0.with_store(id, f)
        }
      }
    }

    impl<R: Runtime> Clone for #name<R> {
      fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
      }
    }
  };

  stream.into()
}
