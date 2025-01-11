use std::env::current_dir;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};
use tauri::test::{mock_app, MockRuntime};
use tauri::Manager;
use tauri_store::{Store, StoreCollection, StoreCollectionBuilder};

pub const STORE_ID: &str = "store";
pub static PATH: LazyLock<PathBuf> = LazyLock::new(path);
pub static CONTEXT: LazyLock<Context> = LazyLock::new(Context::new);

pub struct Context {
  pub collection: Arc<StoreCollection<MockRuntime>>,
}

impl Context {
  fn new() -> Self {
    let app = mock_app();
    let collection = StoreCollectionBuilder::new()
      .path(&*PATH)
      .build(app.app_handle());

    Self { collection }
  }

  pub fn with_store<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&mut Store<MockRuntime>) -> T,
  {
    self.collection.with_store(STORE_ID, f).unwrap()
  }
}

fn path() -> PathBuf {
  current_dir().unwrap().join(".temp")
}
