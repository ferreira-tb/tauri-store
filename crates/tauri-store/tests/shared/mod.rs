use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};
use tauri::test::{mock_app, MockRuntime};
use tauri::Manager;
use tauri_store::{Store, StoreCollection, StoreCollectionBuilder};
use tokio::fs;
use tokio::sync::{OwnedSemaphorePermit as Permit, Semaphore};

pub mod prelude {
  pub use super::{assert_exists, with_store, StoreExt, STORE_ID};
}

pub const STORE_ID: &str = "store";

static PATH: LazyLock<PathBuf> = LazyLock::new(path);
static CONTEXT: LazyLock<Context> = LazyLock::new(Context::new);

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

  async fn acquire_permit(&self) -> Permit {
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
}

pub async fn with_store<F, T>(f: F) -> (T, Permit)
where
  F: FnOnce(&mut Store<MockRuntime>) -> T,
{
  let permit = CONTEXT.acquire_permit().await;
  let value = CONTEXT
    .collection
    .with_store(STORE_ID, f)
    .unwrap();

  (value, permit)
}

pub trait StoreExt {
  fn assert_exists(&self, yes: bool);
}

impl StoreExt for Store<MockRuntime> {
  fn assert_exists(&self, yes: bool) {
    assert_exists(&self.path(), yes);
  }
}

pub fn assert_exists(path: &Path, yes: bool) {
  assert!(path.try_exists().is_ok_and(|it| it == yes));
}

fn path() -> PathBuf {
  current_dir().unwrap().join(".temp")
}
