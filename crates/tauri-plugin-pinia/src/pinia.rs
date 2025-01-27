// This file was autogenerated and should not be edited manually.
// Check the `codegen` command in the `tauri-store-cli` crate.

use std::sync::Arc;
use tauri::Runtime;
use tauri_store::{Collection, StoreCollection};

/// The Pinia plugin.
#[derive(Collection)]
pub struct Pinia<R: Runtime>(pub(crate) Arc<StoreCollection<R>>);
