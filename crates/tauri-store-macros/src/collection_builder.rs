use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[allow(clippy::too_many_lines)]
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

      #[cfg(feature = "unstable-migration")]
      use tauri_store::{Migration, MigrationContext, Migrator};

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
        pub fn save_denylist<I, T>(mut self, denylist: I) -> Self
        where
          I: IntoIterator<Item = T>,
          T: AsRef<str>,
        {
          self
            .save_denylist
            .extend(denylist.into_iter().map(|it| StoreId::from(it.as_ref())));

          self
        }

        /// Sets a list of stores that should not be synchronized across windows.
        #[must_use]
        pub fn sync_denylist<I, T>(mut self, denylist: I) -> Self
        where
          I: IntoIterator<Item = T>,
          T: AsRef<str>,
        {
          self
            .sync_denylist
            .extend(denylist.into_iter().map(|it| StoreId::from(it.as_ref())));

          self
        }

        /// Defines a migration for a store.
        #[must_use]
        #[cfg(feature = "unstable-migration")]
        pub fn migration(mut self, id: impl Into<StoreId>, migration: Migration) -> Self {
          self.migrator.add_migration(id.into(), migration);
          self
        }

        /// Defines multiple migrations for a store.
        #[must_use]
        #[cfg(feature = "unstable-migration")]
        pub fn migrations<I>(mut self, id: impl Into<StoreId>, migrations: I) -> Self
        where
          I: IntoIterator<Item = Migration>,
        {
          self
            .migrator
            .add_migrations(id.into(), migrations);

          self
        }

        /// Sets a closure to be called before each migration step.
        #[must_use]
        #[cfg(feature = "unstable-migration")]
        pub fn on_before_each_migration<F>(mut self, f: F) -> Self
        where
          F: Fn(MigrationContext) + Send + Sync + 'static,
        {
          self.migrator.on_before_each(f);
          self
        }

        pub(super) fn build_collection(mut self, app: &AppHandle<R>) -> Result<Arc<StoreCollection<R>>> {
          let mut collection = StoreCollection::builder()
            .default_save_strategy(self.default_save_strategy)
            .pretty(self.pretty)
            .save_denylist(&self.save_denylist)
            .sync_denylist(&self.sync_denylist);

          if let Some(path) = self.path {
            collection = collection.path(path);
          }

          if let Some(on_load) = self.on_load {
            collection = collection.on_load(on_load);
          }

          if let Some(duration) = self.autosave {
            collection = collection.autosave(duration);
          };

          #[cfg(feature = "unstable-migration")]
          {
            collection = collection.migrator(self.migrator);
          }

          collection.build(app, env!("CARGO_PKG_NAME"))
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

            #[cfg(feature = "unstable-migration")]
            migrator: Migrator::default(),
          }
        }
      }
    }
  };

  stream.into()
}
