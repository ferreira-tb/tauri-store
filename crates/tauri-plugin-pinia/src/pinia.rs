use std::sync::Arc;
use tauri::Runtime;
use tauri_store::{Collection, StoreCollection};

/// The Pinia plugin.
#[derive(Collection)]
pub struct Pinia<R: Runtime>(pub(crate) Arc<StoreCollection<R>>);
