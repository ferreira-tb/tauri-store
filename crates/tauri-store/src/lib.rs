#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://tb.dev.br/tauri-store/favicon.ico")]

mod collection;
mod error;
mod event;
mod manager;
mod meta;
pub mod prelude;
mod store;

#[cfg(feature = "plugin")]
mod command;
#[cfg(feature = "unstable-migration")]
mod migration;
#[cfg(feature = "plugin")]
mod plugin;

pub use collection::{OnLoadFn, StoreCollection};
pub use error::{BoxResult, Error, Result};
pub use manager::ManagerExt;
pub use serde_json::Value as Json;
pub use store::{SaveStrategy, Store, StoreId, StoreOptions, StoreState, WatcherId};

pub use event::{
  EventSource, STORE_CONFIG_CHANGE_EVENT, STORE_STATE_CHANGE_EVENT, STORE_UNLOAD_EVENT,
};

#[cfg(feature = "derive")]
pub use tauri_store_macros::{Collection, CollectionBuilder};

#[cfg(feature = "plugin")]
pub use plugin::{init, Builder};

#[cfg(feature = "unstable-migration")]
pub use migration::{Migration, MigrationContext, Migrator};

use tauri::{Manager, Runtime};

/// Calls a closure with a mutable reference to the store with the given id.
pub fn with_store<R, M, F, T>(manager: &M, id: impl AsRef<str>, f: F) -> Result<T>
where
  R: Runtime,
  M: Manager<R> + ManagerExt<R>,
  F: FnOnce(&mut Store<R>) -> T,
{
  manager.store_collection().with_store(id, f)
}
