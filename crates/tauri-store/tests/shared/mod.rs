#![allow(dead_code)]

use anyhow::Result;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, OnceLock};
use tauri::test::{mock_app, MockRuntime};
use tauri::Manager;
use tauri_store::{Store, StoreCollection, StoreId};
use tokio::fs;
use tokio::sync::{OwnedSemaphorePermit as Permit, Semaphore};

pub static STORE_ID: LazyLock<StoreId> = LazyLock::new(|| StoreId::from("store"));

static TEMP_DIR: OnceLock<PathBuf> = OnceLock::new();
static PATH: LazyLock<PathBuf> = LazyLock::new(default_path);
static CONTEXT: LazyLock<Context> = LazyLock::new(Context::new);

pub struct Context {
  collection: Arc<StoreCollection<MockRuntime>>,
  semaphore: Arc<Semaphore>,
}

impl Context {
  fn new() -> Self {
    let app = mock_app();
    let collection = StoreCollection::builder()
      .path(&*PATH)
      .build(app.app_handle())
      .unwrap();

    Self {
      collection,
      semaphore: Arc::new(Semaphore::new(1)),
    }
  }

  async fn acquire_permit(&self) -> Result<Permit> {
    let permit = Arc::clone(&self.semaphore)
      .acquire_owned()
      .await?;

    self.collection.unload_store(&STORE_ID)?;

    let temp_dir = temp_dir();
    if fs::try_exists(temp_dir).await? {
      fs::remove_dir_all(temp_dir).await?;
    }

    Ok(permit)
  }
}

pub async fn with_collection<F, T>(f: F) -> (T, Permit)
where
  F: FnOnce(&StoreCollection<MockRuntime>) -> T,
{
  let permit = CONTEXT.acquire_permit().await.unwrap();
  let value = f(&CONTEXT.collection);
  (value, permit)
}

pub async fn with_store<F, T>(f: F) -> (T, Permit)
where
  F: FnOnce(&mut Store<MockRuntime>) -> T,
{
  let permit = CONTEXT.acquire_permit().await.unwrap();
  let value = CONTEXT
    .collection
    .with_store(&*STORE_ID, f)
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

pub fn temp_dir() -> &'static Path {
  TEMP_DIR.get_or_init(|| current_dir().unwrap().join(".temp"))
}

fn default_path() -> PathBuf {
  temp_dir().join(env!("CARGO_PKG_NAME"))
}
