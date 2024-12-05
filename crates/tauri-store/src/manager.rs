use crate::collection::StoreCollection;
use crate::collection::RESOURCE_ID;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow, Window};

pub trait ManagerExt<R: Runtime>: Manager<R> {
  fn store_collection(&self) -> Arc<StoreCollection<R>> {
    let rid = RESOURCE_ID
      .get()
      .expect("missing store collection resource id");

    self
      .resources_table()
      .get::<StoreCollection<R>>(*rid)
      .expect("store collection is not in the resources table")
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
