use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_collection_builder(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let stream = quote! {
    mod __impl_collection_builder {
      use super::#name;
      use std::path::Path;
      use std::time::Duration;
      use tauri::Runtime;
      use tauri_store::{OnLoadResult, SaveStrategy, Store};

      impl<R: Runtime> #name<R> {
        /// Creates a new builder instance with default values.
        pub fn new() -> Self {
          Self::default()
        }

        /// Directory where the stores will be saved.
        #[must_use]
        pub fn path(mut self, path: impl AsRef<Path>) -> Self {
          let path = path.as_ref().to_path_buf();
          self.path = Some(path);
          self
        }

        /// Sets the default save strategy to be used by the stores.
        #[must_use]
        pub fn default_save_strategy(mut self, strategy: SaveStrategy) -> Self {
          self.default_save_strategy = strategy;
          self
        }

        /// Sets the autosave interval for all stores.
        #[must_use]
        pub fn autosave(mut self, interval: Duration) -> Self {
          self.autosave = Some(interval);
          self
        }

        /// Sets a function to be called when a store is loaded.
        #[must_use]
        pub fn on_load<F>(mut self, f: F) -> Self
        where
          F: Fn(&Store<R>) -> OnLoadResult + Send + Sync + 'static,
        {
          self.on_load = Some(Box::new(f));
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
            .extend(denylist.iter().map(|s| s.as_ref().to_string()));

          self
        }

        /// Sets a list of stores that should not be synchronized across windows.
        #[must_use]
        pub fn sync_denylist(mut self, denylist: &[impl AsRef<str>]) -> Self {
          self
            .sync_denylist
            .extend(denylist.iter().map(|s| s.as_ref().to_string()));

          self
        }
      }
    }
  };

  stream.into()
}
