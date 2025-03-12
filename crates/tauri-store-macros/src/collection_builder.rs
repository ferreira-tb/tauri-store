use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_collection_builder(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_collection_builder {
      use super::#name;
      use std::collections::HashSet;
      use std::path::Path;
      use std::sync::Arc;
      use std::time::Duration;
      use tauri::{AppHandle, Manager as _, Runtime};
      use tauri_store::prelude::*;

      impl<R: Runtime> #name<R> {
        /// Creates a new builder instance with default values.
        pub fn new() -> Self {
          Self::default()
        }

        /// Sets the autosave interval for all stores.
        #[must_use]
        pub fn autosave(mut self, interval: Duration) -> Self {
          self.autosave = Some(interval);
          self
        }

        /// Sets the default save strategy to be used by the stores.
        #[must_use]
        pub fn default_save_strategy(mut self, strategy: SaveStrategy) -> Self {
          self.default_save_strategy = strategy;
          self
        }

        /// Registers a closure to be called when a store is loaded.
        #[must_use]
        pub fn on_load<F>(mut self, f: F) -> Self
        where
          F: Fn(&Store<R>) -> Result<()> + Send + Sync + 'static,
        {
          self.on_load = Some(Box::new(f));
          self
        }

        /// Directory where the stores will be saved.
        #[must_use]
        pub fn path(mut self, path: impl AsRef<Path>) -> Self {
          let path = path.as_ref().to_path_buf();
          self.path = Some(path);
          self
        }

        /// Sets whether the store files should be pretty printed.
        #[must_use]
        pub fn pretty(mut self, yes: bool) -> Self {
          self.pretty = yes;
          self
        }

        /// Sets a list of stores that should not be saved to disk.
        #[must_use]
        pub fn save_denylist(mut self, denylist: &[impl AsRef<str>]) -> Self {
          self
            .save_denylist
            .extend(denylist.iter().map(|it| StoreId::from(it.as_ref())));

          self
        }

        /// Sets a list of stores that should not be synchronized across windows.
        #[must_use]
        pub fn sync_denylist(mut self, denylist: &[impl AsRef<str>]) -> Self {
          self
            .sync_denylist
            .extend(denylist.iter().map(|it| StoreId::from(it.as_ref())));

          self
        }

        pub(super) fn build_collection(mut self, app: &AppHandle<R>) -> Result<Arc<StoreCollection<R>>> {
          let path = self.path.take().unwrap_or_else(|| {
            app
              .path()
              .app_data_dir()
              .expect("failed to resolve app data dir")
              .join(env!("CARGO_PKG_NAME"))
          });

          let mut collection = StoreCollection::builder()
            .path(path)
            .default_save_strategy(self.default_save_strategy)
            .pretty(self.pretty)
            .save_denylist(self.save_denylist)
            .sync_denylist(self.sync_denylist);

          if let Some(on_load) = self.on_load {
            collection = collection.on_load(on_load);
          }

          if let Some(duration) = self.autosave {
            collection = collection.autosave(duration);
          };

          collection.build(app)
        }
      }

      impl<R: Runtime> Default for #name<R> {
        fn default() -> Self {
          Self {
            path: None,
            default_save_strategy: SaveStrategy::default(),
            autosave: None,
            on_load: None,
            pretty: false,
            save_denylist: HashSet::default(),
            sync_denylist: HashSet::default(),
          }
        }
      }
    }
  };

  stream.into()
}
