use std::env::current_dir;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};
use tauri::test::{mock_app, MockRuntime};
use tauri::Manager;
use tauri_store::{Store, StoreCollection, StoreCollectionBuilder};
use tokio::fs;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub const STORE_ID: &str = "store";
pub static PATH: LazyLock<PathBuf> = LazyLock::new(path);
pub static CONTEXT: LazyLock<Context> = LazyLock::new(Context::new);

pub struct Context {
  collection: Arc<StoreCollection<MockRuntime>>,
  semaphore: Arc<Semaphore>,
}

impl Context {
  fn new() -> Self {
    let app = mock_app();
    let collection = StoreCollectionBuilder::new()
      .path(&*PATH)
      .build(app.app_handle());

    Self {
      collection,
      semaphore: Arc::new(Semaphore::new(1)),
    }
  }

  pub async fn acquire_permit(&self) -> OwnedSemaphorePermit {
    let permit = Arc::clone(&self.semaphore)
      .acquire_owned()
      .await
      .unwrap();

    self.collection.unload_store(STORE_ID).unwrap();

    let path = self.collection.path();
    if let Ok(true) = fs::try_exists(path).await {
      fs::remove_dir_all(path).await.unwrap();
    }

    permit
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
