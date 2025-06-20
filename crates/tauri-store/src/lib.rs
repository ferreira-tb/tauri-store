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

pub use collection::{CollectionMarker, DefaultMarker, OnLoadFn, StoreCollection};
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
