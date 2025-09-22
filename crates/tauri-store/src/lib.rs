#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://tb.dev.br/tauri-store/favicon.ico")]

mod collection;
mod error;
mod event;
mod manager;
mod migration;
pub mod prelude;
mod store;

#[cfg(feature = "plugin")]
mod command;

#[cfg(feature = "plugin")]
mod plugin;

pub use collection::{CollectionMarker, DefaultMarker, Handle, OnLoadFn, StoreCollection};
pub use error::{BoxResult, Error, Result};
pub use event::{
  EventSource, STORE_CONFIG_CHANGE_EVENT, STORE_STATE_CHANGE_EVENT, STORE_UNLOAD_EVENT,
};
pub use manager::ManagerExt;
pub use migration::{Migration, MigrationContext, Migrator};
pub use serde_json::Value as Json;
pub use store::{
  JsonMarshaler, Marshaler, MarshalingError, SaveStrategy, Store, StoreId, StoreOptions,
  StoreState, WatcherId,
};

#[cfg(feature = "derive")]
pub use tauri_store_macros::Collection;

#[cfg(feature = "plugin")]
pub use plugin::{init, Builder};

#[cfg(feature = "marshaler-toml")]
pub use store::TomlMarshaler;
