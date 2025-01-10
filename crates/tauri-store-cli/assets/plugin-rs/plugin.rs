use std::sync::Arc;
use tauri::Runtime;
use tauri_store::{Collection, StoreCollection};

/// The PASCAL_PLUGIN_NAME plugin.
#[derive(Collection)]
pub struct PASCAL_PLUGIN_NAME<R: Runtime>(pub(crate) Arc<StoreCollection<R>>);
