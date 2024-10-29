use std::sync::Arc;
use tauri::Runtime;
use tauri_store::{Collection, StoreCollection};

#[derive(Collection)]
pub struct Pinia<R: Runtime>(pub(crate) Arc<StoreCollection<R>>);
