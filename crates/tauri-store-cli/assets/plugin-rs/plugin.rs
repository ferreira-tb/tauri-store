use tauri::{Runtime, State};
use tauri_store::{Collection, CollectionMarker, StoreCollection};

/// The __PASCAL_PLUGIN_TITLE__ plugin.
#[derive(Collection)]
pub struct __PASCAL_PLUGIN_TITLE__<'a, R: Runtime>(
  pub(crate) State<'a, StoreCollection<R, __PASCAL_PLUGIN_TITLE__Marker>>,
);

/// Marker for the __PASCAL_PLUGIN_TITLE__ plugin.
pub struct __PASCAL_PLUGIN_TITLE__Marker;

impl CollectionMarker for __PASCAL_PLUGIN_TITLE__Marker {}
