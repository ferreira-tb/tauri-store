use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[expect(clippy::too_many_lines)]
#[proc_macro_derive(Collection)]
pub fn derive_collection(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  let name = ast.ident;

  let stream = quote! {
    impl<R: tauri::Runtime> #name<R> {
      /// Directory where the stores are saved.
      pub fn path(&self) -> &std::path::Path {
        &self.0.path()
      }

      /// Calls a closure with a mutable reference to the store with the given id.
      #[cfg(not(feature = "unstable-async"))]
      pub fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> tauri_store::Result<T>
      where
        F: FnOnce(&mut tauri_store::Store<R>) -> tauri_store::Result<T>,
      {
        self.0.with_store(id, f)
      }

      /// Calls a closure with a mutable reference to the store with the given id.
      #[cfg(feature = "unstable-async")]
      pub fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> tauri_store::BoxFuture<tauri_store::Result<T>>
      where
        F: FnOnce(&mut tauri_store::Store<R>) -> tauri_store::BoxFuture<tauri_store::Result<T>> + Send + 'static,
        T: Send + 'static,
      {
        self.0.with_store(id, f)
      }

      /// Saves a store to the disk.
      #[cfg(not(feature = "unstable-async"))]
      pub fn save(&self, id: impl AsRef<str>) -> tauri_store::Result<()> {
        self.0.save(id)
      }

      /// Saves a store to the disk.
      #[cfg(feature = "unstable-async")]
      pub async fn save(&self, id: impl AsRef<str>) -> tauri_store::Result<()> {
        self.0.save(id).await
      }

      /// Saves some stores to the disk.
      #[cfg(not(feature = "unstable-async"))]
      pub fn save_some(&self, ids: &[impl AsRef<str>]) -> tauri_store::Result<()> {
        self.0.save_some(ids)
      }

      /// Saves some stores to the disk.
      #[cfg(feature = "unstable-async")]
      pub async fn save_some(&self, ids: &[impl AsRef<str>]) -> tauri_store::Result<()> {
        self.0.save_some(ids).await
      }

      /// Saves all the stores to the disk.
      #[cfg(not(feature = "unstable-async"))]
      pub fn save_all(&self) -> tauri_store::Result<()> {
        self.0.save_all()
      }

      /// Saves all the stores to the disk.
      #[cfg(feature = "unstable-async")]
      pub async fn save_all(&self) -> tauri_store::Result<()> {
        self.0.save_all().await
      }

      /// Lists all the store ids.
      #[cfg(not(feature = "unstable-async"))]
      pub fn ids(&self) -> Vec<String> {
        self.0.ids()
      }

      /// Lists all the store ids.
      #[cfg(feature = "unstable-async")]
      pub async fn ids(&self) -> Vec<String> {
        self.0.ids().await
      }

      /// Gets a clone of the store state if it exists.
      ///
      /// **WARNING:** Changes to the returned state will not be reflected in the store.
      #[cfg(not(feature = "unstable-async"))]
      pub fn store_state(&self, store_id: impl AsRef<str>) -> Option<tauri_store::StoreState> {
        self.0.store_state(store_id)
      }

      /// Gets a clone of the store state if it exists.
      ///
      /// **WARNING:** Changes to the returned state will not be reflected in the store.
      #[cfg(feature = "unstable-async")]
      pub async fn store_state(&self, store_id: impl AsRef<str>) -> Option<tauri_store::StoreState> {
        self.0.store_state(store_id).await
      }

      /// Gets the store state if it exists, then tries to parse it as an instance of type `T`.
      #[cfg(not(feature = "unstable-async"))]
      pub fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> tauri_store::Result<T>
      where
        T: serde::de::DeserializeOwned,
      {
        self.0.try_store_state(store_id)
      }

      /// Gets the store state if it exists, then tries to parse it as an instance of type `T`.
      #[cfg(feature = "unstable-async")]
      pub async fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> tauri_store::Result<T>
      where
        T: serde::de::DeserializeOwned,
      {
        self.0.try_store_state(store_id).await
      }

      /// Gets a value from a store.
      #[cfg(not(feature = "unstable-async"))]
      pub fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<tauri_store::Json> {
        self.0.get(store_id, key)
      }

      /// Gets a value from a store.
      #[cfg(feature = "unstable-async")]
      pub async fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<tauri_store::Json> {
        self.0.get(store_id, key).await
      }

      #[cfg(not(feature = "unstable-async"))]
      /// Gets a value from a store and tries to parse it as an instance of type `T`.
      pub fn try_get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> tauri_store::Result<T>
      where
        T: serde::de::DeserializeOwned,
      {
        self.0.try_get(store_id, key)
      }

      #[cfg(feature = "unstable-async")]
      /// Gets a value from a store and tries to parse it as an instance of type `T`.
      pub async fn try_get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> tauri_store::Result<T>
      where
        T: serde::de::DeserializeOwned,
      {
        self.0.try_get(store_id, key).await
      }

      /// Sets a key-value pair in a store.
      #[cfg(not(feature = "unstable-async"))]
      pub fn set(&self, id: impl AsRef<str>, key: impl AsRef<str>, value: tauri_store::Json) -> tauri_store::Result<()> {
        self.0.set(id, key, value)
      }

      /// Sets a key-value pair in a store.
      #[cfg(feature = "unstable-async")]
      pub async fn set(&self, id: impl AsRef<str>, key: impl AsRef<str>, value: tauri_store::Json) -> tauri_store::Result<()> {
        self.0.set(id, key, value).await
      }

      /// Patches a store state.
      #[cfg(not(feature = "unstable-async"))]
      pub fn patch(&self, store_id: impl AsRef<str>, state: tauri_store::StoreState) -> tauri_store::Result<()> {
        self.0.patch(store_id, state)
      }

      /// Patches a store state.
      #[cfg(feature = "unstable-async")]
      pub async fn patch(&self, store_id: impl AsRef<str>, state: tauri_store::StoreState) -> tauri_store::Result<()> {
        self.0.patch(store_id, state).await
      }

      /// Watches a store for changes.
      #[cfg(not(feature = "unstable-async"))]
      pub fn watch<F>(&self, store_id: impl AsRef<str>, f: F) -> tauri_store::Result<u32>
      where
        F: Fn(std::sync::Arc<tauri_store::StoreState>) -> tauri_store::WatcherResult + Send + Sync + 'static,
      {
        self.0.watch(store_id, f)
      }

      /// Watches a store for changes.
      #[cfg(feature = "unstable-async")]
      pub async fn watch<F>(&self, store_id: impl AsRef<str>, f: F) -> tauri_store::Result<u32>
      where
        F: Fn(std::sync::Arc<tauri_store::StoreState>) -> tauri_store::WatcherResult + Send + Sync + 'static,
      {
        self.0.watch(store_id, f).await
      }

      /// Removes a listener from a store.
      #[cfg(not(feature = "unstable-async"))]
      pub fn unwatch(&self, store_id: impl AsRef<str>, listener_id: u32) -> tauri_store::Result<bool> {
        self.0.unwatch(store_id, listener_id)
      }

      /// Removes a listener from a store.
      #[cfg(feature = "unstable-async")]
      pub async fn unwatch(&self, store_id: impl AsRef<str>, listener_id: u32) -> tauri_store::Result<bool> {
        self.0.unwatch(store_id, listener_id).await
      }

      /// Saves the stores periodically.
      #[cfg(feature = "unstable-async")]
      #[cfg_attr(docsrs, doc(cfg(feature = "unstable-async")))]
      pub fn set_autosave(&self, duration: std::time::Duration) {
        self.0.set_autosave(duration)
      }

      /// Stops the autosave.
      #[cfg(feature = "unstable-async")]
      #[cfg_attr(docsrs, doc(cfg(feature = "unstable-async")))]
      pub fn clear_autosave(&self) {
        self.0.clear_autosave()
      }

      #[cfg(not(feature = "unstable-async"))]
      pub(crate) fn unload_store(&self, id: &str) -> tauri_store::Result<()> {
        self.0.unload_store(id)
      }

      #[cfg(feature = "unstable-async")]
      pub(crate) async fn unload_store(&self, id: &str) -> tauri_store::Result<()> {
        self.0.unload_store(id).await
      }
    }

    impl<R: tauri::Runtime> Clone for #name<R> {
      fn clone(&self) -> Self {
        Self(std::sync::Arc::clone(&self.0))
      }
    }
  };

  stream.into()
}
