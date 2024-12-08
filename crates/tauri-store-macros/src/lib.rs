#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Ident};

#[proc_macro_derive(Collection)]
pub fn derive_collection(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  let name = ast.ident;
  let sync = impl_sync(&name);
  let unstable_async = impl_unstable_async(&name);

  let stream = quote! {
    impl<R: tauri::Runtime> #name<R> {
      /// Directory where the stores are saved.
      pub fn path(&self) -> &std::path::Path {
        &self.0.path()
      }

      /// Lists all the store ids.
      pub fn ids(&self) -> Vec<String> {
        self.0.ids()
      }

      /// Current save strategy for the stores.
      pub fn save_strategy(&self) -> tauri_store::SaveStrategy {
        self.0.save_strategy()
      }

      /// Saves the stores periodically.
      pub fn set_autosave(&self, duration: std::time::Duration) {
        self.0.set_autosave(duration)
      }

      /// Stops the autosave.
      pub fn clear_autosave(&self) {
        self.0.clear_autosave()
      }
    }

    #sync
    #unstable_async

    impl<R: tauri::Runtime> Clone for #name<R> {
      fn clone(&self) -> Self {
        Self(std::sync::Arc::clone(&self.0))
      }
    }
  };

  stream.into()
}

fn impl_sync(name: &Ident) -> TokenStream2 {
  quote! {
    #[cfg(not(feature = "unstable-async"))]
    impl<R: tauri::Runtime> #name<R> {
      /// Calls a closure with a mutable reference to the store with the given id.
      pub fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> tauri_store::Result<T>
      where
        F: FnOnce(&mut tauri_store::Store<R>) -> T,
      {
        self.0.with_store(id, f)
      }

      /// Saves a store to the disk.
      pub fn save(&self, id: impl AsRef<str>) -> tauri_store::Result<()> {
        self.0.save(id)
      }

      /// Saves some stores to the disk.
      pub fn save_some(&self, ids: &[impl AsRef<str>]) -> tauri_store::Result<()> {
        self.0.save_some(ids)
      }

      /// Saves all the stores to the disk.
      pub fn save_all(&self) -> tauri_store::Result<()> {
        self.0.save_all()
      }

      /// Gets a clone of the store state if it exists.
      pub fn store_state(&self, store_id: impl AsRef<str>) -> Option<tauri_store::StoreState> {
        self.0.store_state(store_id)
      }

      /// Gets the store state if it exists, then tries to parse it as an instance of type `T`.
      pub fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> tauri_store::Result<T>
      where
        T: serde::de::DeserializeOwned,
      {
        self.0.try_store_state(store_id)
      }

      /// Gets a value from a store.
      pub fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<tauri_store::Json> {
        self.0.get(store_id, key)
      }

      /// Gets a value from a store and tries to parse it as an instance of type `T`.
      pub fn try_get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> tauri_store::Result<T>
      where
        T: serde::de::DeserializeOwned,
      {
        self.0.try_get(store_id, key)
      }

      /// Sets a key-value pair in a store.
      pub fn set(&self, id: impl AsRef<str>, key: impl AsRef<str>, value: tauri_store::Json) -> tauri_store::Result<()> {
        self.0.set(id, key, value)
      }

      /// Patches a store state.
      pub fn patch(&self, store_id: impl AsRef<str>, state: tauri_store::StoreState) -> tauri_store::Result<()> {
        self.0.patch(store_id, state)
      }

      /// Watches a store for changes.
      pub fn watch<F>(&self, store_id: impl AsRef<str>, f: F) -> tauri_store::Result<u32>
      where
        F: Fn(tauri::AppHandle<R>) -> tauri_store::WatcherResult + Send + Sync + 'static,
      {
        self.0.watch(store_id, f)
      }

      /// Removes a listener from a store.
      pub fn unwatch(&self, store_id: impl AsRef<str>, listener_id: u32) -> tauri_store::Result<bool> {
        self.0.unwatch(store_id, listener_id)
      }

      pub(crate) fn unload_store(&self, id: &str) -> tauri_store::Result<()> {
        self.0.unload_store(id)
      }
    }
  }
}

fn impl_unstable_async(name: &Ident) -> TokenStream2 {
  quote! {
    #[cfg(feature = "unstable-async")]
    impl<R: tauri::Runtime> #name<R> {
      /// Calls a closure with a mutable reference to the store with the given id.
      pub async fn with_store<F, Fut, T>(&self, id: impl AsRef<str>, f: F) -> tauri_store::Result<T>
      where
        F: FnOnce(&mut tauri_store::Store<R>) -> Fut + Send,
        Fut: std::future::Future<Output = T> + Send,
        T: Send + 'static,
      {
        self.0.with_store(id, f).await
      }

      /// Saves a store to the disk.
      pub async fn save(&self, id: impl AsRef<str>) -> tauri_store::Result<()> {
        self.0.save(id).await
      }

      /// Saves some stores to the disk.
      pub async fn save_some(&self, ids: &[impl AsRef<str>]) -> tauri_store::Result<()> {
        self.0.save_some(ids).await
      }

      /// Saves all the stores to the disk.
      pub async fn save_all(&self) -> tauri_store::Result<()> {
        self.0.save_all().await
      }

      /// Gets a clone of the store state if it exists.
      pub async fn store_state(&self, store_id: impl AsRef<str>) -> Option<tauri_store::StoreState> {
        self.0.store_state(store_id).await
      }

      /// Gets the store state if it exists, then tries to parse it as an instance of type `T`.
      pub async fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> tauri_store::Result<T>
      where
        T: serde::de::DeserializeOwned,
      {
        self.0.try_store_state(store_id).await
      }

      /// Gets a value from a store.
      pub async fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<tauri_store::Json> {
        self.0.get(store_id, key).await
      }

      /// Gets a value from a store and tries to parse it as an instance of type `T`.
      pub async fn try_get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> tauri_store::Result<T>
      where
        T: serde::de::DeserializeOwned,
      {
        self.0.try_get(store_id, key).await
      }

      /// Sets a key-value pair in a store.
      pub async fn set(&self, id: impl AsRef<str>, key: impl AsRef<str>, value: tauri_store::Json) -> tauri_store::Result<()> {
        self.0.set(id, key, value).await
      }

      /// Patches a store state.
      pub async fn patch(&self, store_id: impl AsRef<str>, state: tauri_store::StoreState) -> tauri_store::Result<()> {
        self.0.patch(store_id, state).await
      }

      /// Watches a store for changes.
      pub async fn watch<F>(&self, store_id: impl AsRef<str>, f: F) -> tauri_store::Result<u32>
      where
        F: Fn(tauri::AppHandle<R>) -> tauri_store::WatcherResult + Send + Sync + 'static,
      {
        self.0.watch(store_id, f).await
      }

      /// Removes a listener from a store.
      pub async fn unwatch(&self, store_id: impl AsRef<str>, listener_id: u32) -> tauri_store::Result<bool> {
        self.0.unwatch(store_id, listener_id).await
      }

      pub(crate) async fn unload_store(&self, id: &str) -> tauri_store::Result<()> {
        self.0.unload_store(id).await
      }
    }
  }
}
