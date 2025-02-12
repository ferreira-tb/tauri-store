use std::sync::Arc;
use tauri::Runtime;
use tauri_store::{Collection, StoreCollection};

/// The __PASCAL_PLUGIN_NAME__ plugin.
#[derive(Collection)]
pub struct __PASCAL_PLUGIN_NAME__<R: Runtime>(pub(crate) Arc<StoreCollection<R>>);
