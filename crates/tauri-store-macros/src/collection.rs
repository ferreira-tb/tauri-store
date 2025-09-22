use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::DeriveInput;

#[allow(clippy::too_many_lines)]
pub fn impl_collection(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let marker = format!("{name}Marker");
  let marker_ident = Ident::new(&marker, Span::call_site());

  let stream = quote! {
    mod __impl_collection {
      use super::{#name, #marker_ident};
      use serde::de::DeserializeOwned;
      use std::path::Path;
      use std::time::Duration;
      use tauri::{AppHandle, Runtime};
      use tauri_store::prelude::*;

      impl<'a, R: Runtime> #name<'a, R> {
        /// Lists all the store ids.
        pub fn ids(&self) -> Vec<StoreId> {
          self.0.ids()
        }

        /// Directory where the stores are saved.
        pub fn path(&self) -> &Path {
          self.0.path()
        }

        /// Calls a closure with a mutable reference to the store with the given id.
        pub fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
        where
          F: FnOnce(&mut Store<R, #marker_ident>) -> T,
        {
          self.0.with_store(id, f)
        }

        /// Gets a clone of the raw store state if it exists.
        pub fn raw_state(&self, store_id: impl AsRef<str>) -> Result<StoreState> {
          self.0.raw_state(store_id)
        }

        /// Gets the store state, then tries to parse it as an instance of type `T`.
        pub fn state<T>(&self, store_id: impl AsRef<str>) -> Result<T>
        where
          T: DeserializeOwned,
        {
          self.0.state(store_id)
        }

        /// Gets the store state, then tries to parse it as an instance of type `T`.
        ///
        /// If it cannot be parsed, returns the provided default value.
        pub fn state_or<T>(&self, store_id: impl AsRef<str>, default: T) -> Result<T>
        where
          T: DeserializeOwned,
        {
          self.0.state_or(store_id, default)
        }

        /// Gets the store state, then tries to parse it as an instance of type `T`.
        ///
        /// If it cannot be parsed, returns the default value of `T`.
        pub fn state_or_default<T>(&self, store_id: impl AsRef<str>) -> Result<T>
        where
          T: DeserializeOwned + Default,
        {
          self.0.state_or_default(store_id)
        }

        /// Gets the store state, then tries to parse it as an instance of type `T`.
        ///
        /// If it cannot be parsed, returns the result of the provided closure.
        pub fn state_or_else<T>(&self, store_id: impl AsRef<str>, f: impl FnOnce() -> T) -> Result<T>
        where
          T: DeserializeOwned,
        {
          self.0.state_or_else(store_id, f)
        }

        /// Gets a raw value from a store.
        pub fn get_raw(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<Json> {
          self.0.get_raw(store_id, key)
        }

        /// Gets a raw value from a store.
        ///
        /// # Safety
        ///
        /// This is *undefined behavior* if the key doesn't exist in the store.
        pub unsafe fn get_raw_unchecked(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Json {
          unsafe { self.0.get_raw_unchecked(store_id, key) }
        }

        /// Gets a value from a store and tries to parse it as an instance of type `T`.
        pub fn get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Result<T>
        where
          T: DeserializeOwned,
        {
          self.0.get(store_id, key)
        }

        /// Gets a value from a store and tries to parse it as an instance of type `T`.
        ///
        /// If the key does not exist, returns the provided default value.
        pub fn get_or<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>, default: T) -> T
        where
          T: DeserializeOwned,
        {
          self.0.get_or(store_id, key, default)
        }

        /// Gets a value from a store and tries to parse it as an instance of type `T`.
        ///
        /// If the key does not exist, returns the default value of `T`.
        pub fn get_or_default<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> T
        where
          T: Default + DeserializeOwned,
        {
          self.0.get_or_default(store_id, key)
        }

        /// Gets a value from a store and tries to parse it as an instance of type `T`.
        ///
        /// If the key does not exist, returns the result of the provided closure.
        pub fn get_or_else<T, F>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>, f: F) -> T
        where
          T: DeserializeOwned,
          F: FnOnce() -> T,
        {
          self.0.get_or_else(store_id, key, f)
        }

        /// Gets a value from a store and parses it as an instance of type `T`.
        ///
        /// # Safety
        ///
        /// This is *undefined behavior* if the key doesn't exist in the store
        /// **OR** if the value cannot be represented as a valid `T`.
        pub unsafe fn get_unchecked<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> T
        where
          T: DeserializeOwned,
        {
          unsafe { self.0.get(store_id, key).unwrap_unchecked() }
        }

        /// Sets a key-value pair in a store.
        pub fn set<K, V>(&self, id: impl AsRef<str>, key: K, value: V) -> Result<()>
        where
          K: AsRef<str>,
          V: Into<Json>,
        {
          self.0.set(id, key, value)
        }

        /// Patches a store state.
        pub fn patch<S>(&self, store_id: impl AsRef<str>, state: S) -> Result<()>
        where
          S: Into<StoreState>,
        {
          self.0.patch(store_id, state)
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

        /// Default save strategy for the stores.
        /// This can be overridden on a per-store basis.
        pub fn default_save_strategy(&self) -> SaveStrategy {
          self.0.default_save_strategy()
        }

        /// Saves the stores periodically.
        pub fn set_autosave(&self, duration: Duration) {
          self.0.set_autosave(duration)
        }

        /// Stops the autosave.
        pub fn clear_autosave(&self) {
          self.0.clear_autosave()
        }

        /// Watches a store for changes.
        pub fn watch<F>(&self, store_id: impl AsRef<str>, f: F) -> Result<WatcherId>
        where
          F: Fn(AppHandle<R>) -> Result<()> + Send + Sync + 'static,
        {
          self.0.watch(store_id, f)
        }

        /// Removes a watcher from a store.
        pub fn unwatch(&self, store_id: impl AsRef<str>, watcher_id: impl Into<WatcherId>) -> Result<bool> {
          self.0.unwatch(store_id, watcher_id)
        }

        /// Removes a store from the save denylist.
        pub fn allow_save(&self, id: impl AsRef<str>) {
          self.0.allow_save(id);
        }

        /// Adds a store to the save denylist.
        pub fn deny_save(&self, id: impl AsRef<str>) {
          self.0.deny_save(id);
        }

        /// Removes a store from the sync denylist.
        pub fn allow_sync(&self, id: impl AsRef<str>) {
          self.0.allow_sync(id);
        }

        /// Adds a store to the deny denylist.
        pub fn deny_sync(&self, id: impl AsRef<str>) {
          self.0.deny_sync(id);
        }

        /// Destroys a store, cleans up its state, and deletes its file.
        pub fn destroy(&self, id: impl AsRef<str>) -> Result<()> {
          self.0.destroy(id)
        }

        pub(crate) fn unload_store(&self, id: &StoreId) -> Result<()> {
          self.0.unload_store(id)
        }
      }
    }
  };

  stream.into()
}
