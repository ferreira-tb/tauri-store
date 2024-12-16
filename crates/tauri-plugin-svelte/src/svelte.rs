use std::sync::Arc;
use tauri::Runtime;
use tauri_store::{Collection, StoreCollection};

/// The Svelte plugin.
#[derive(Collection)]
pub struct Svelte<R: Runtime>(pub(crate) Arc<StoreCollection<R>>);
