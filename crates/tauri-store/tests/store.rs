#![allow(unused_must_use)]

use anyhow::Result;
use itertools::Itertools;
use serde::Deserialize;
use serde_json::Value;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, OnceLock};
use tauri::test::{mock_app, MockRuntime};
use tauri::{AppHandle, Manager};
use tauri_store::{DefaultMarker, ManagerExt, SaveStrategy, Store, StoreCollection, StoreId};
use tokio::fs;
use tokio::sync::{Notify, Semaphore, SemaphorePermit};
use tokio::time::{sleep, timeout, Duration};

static STORE_ID: LazyLock<StoreId> = LazyLock::new(|| StoreId::from("store"));

static TEMP_DIR: OnceLock<PathBuf> = OnceLock::new();
static PATH: LazyLock<PathBuf> = LazyLock::new(default_path);

static SEMAPHORE: Semaphore = Semaphore::const_new(1);
static HANDLE: LazyLock<AppHandle<MockRuntime>> = LazyLock::new(|| {
  let app = mock_app();
  let handle = app.app_handle();
  StoreCollection::<_, DefaultMarker>::builder()
    .path(&*PATH)
    .build(app.app_handle(), env!("CARGO_PKG_NAME"))
    .unwrap();

  handle.clone()
});

#[derive(Default, Deserialize)]
struct Foo {
  key: u8,
}

#[tokio::test]
async fn id() {
  with_store(|store| assert_eq!(store.id(), *STORE_ID)).await;
}

#[tokio::test]
async fn try_state() {
  with_store(|store| {
    let state = store.try_state::<Foo>();
    assert!(state.is_err());

    store.set("key", 42).unwrap();
    let state = store.try_state::<Foo>();
    assert!(state.is_ok_and(|state| state.key == 42));
  })
  .await;
}

#[tokio::test]
async fn try_state_or() {
  with_store(|store| {
    let state = store.try_state_or(Foo { key: 100 });
    assert_eq!(state.key, 100);

    store.set("key", 42).unwrap();
    let state = store.try_state_or(Foo { key: 100 });
    assert_eq!(state.key, 42);
  })
  .await;
}

#[tokio::test]
async fn try_state_or_default() {
  with_store(|store| {
    let state = store.try_state_or_default::<Foo>();
    assert_eq!(state.key, 0);

    store.set("key", 42).unwrap();
    let state = store.try_state_or_default::<Foo>();
    assert_eq!(state.key, 42);
  })
  .await;
}

#[tokio::test]
async fn try_state_or_else() {
  with_store(|store| {
    let else_fn = || Foo { key: 100 };
    let state = store.try_state_or_else(else_fn);
    assert_eq!(state.key, 100);

    store.set("key", 42).unwrap();
    let state = store.try_state_or_else(else_fn);
    assert_eq!(state.key, 42);
  })
  .await;
}

#[tokio::test]
async fn get() {
  with_store(|store| {
    let value = store.get("key");
    assert!(value.is_none());

    store.set("key", "value").unwrap();
    let value = store.get("key").unwrap();
    assert_eq!(value, &Value::from("value"));
  })
  .await;
}

#[tokio::test]
async fn try_get() {
  with_store(|store| {
    let value = store.try_get::<u8>("key");
    assert!(value.is_err());

    store.set("key", 42).unwrap();
    let value = store.try_get::<u8>("key");
    assert_eq!(value.unwrap(), 42);
  })
  .await;
}

#[tokio::test]
async fn try_get_or() {
  with_store(|store| {
    let value = store.try_get_or::<u8>("key", 20);
    assert_eq!(value, 20);

    store.set("key", 42).unwrap();
    let value = store.try_get_or::<u8>("key", 20);
    assert_eq!(value, 42);
  })
  .await;
}

#[tokio::test]
async fn try_get_or_default() {
  with_store(|store| {
    let value = store.try_get_or_default::<u8>("key");
    assert_eq!(value, 0);

    store.set("key", 42).unwrap();
    let value = store.try_get_or_default::<u8>("key");
    assert_eq!(value, 42);
  })
  .await;
}

#[tokio::test]
async fn try_get_or_else() {
  let else_fn = || 20;
  with_store(|store| {
    let value = store.try_get_or_else::<u8>("key", else_fn);
    assert_eq!(value, 20);

    store.set("key", 42).unwrap();
    let value = store.try_get_or_else::<u8>("key", else_fn);
    assert_eq!(value, 42);
  })
  .await;
}

#[tokio::test]
async fn set() {
  with_store(|store| {
    let value = store.get("key");
    assert!(value.is_none());

    store.set("key", 42).unwrap();
    let value = store.get("key").unwrap();
    assert_eq!(value, &Value::from(42));
  })
  .await;
}

#[tokio::test]
async fn patch() {
  with_store(|store| {
    let value = store.get("key");
    assert!(value.is_none());

    store.patch(("key", 42)).unwrap();
    let value = store.get("key").unwrap();
    assert_eq!(value, &Value::from(42));
  })
  .await;
}

#[tokio::test]
async fn patch_many() {
  with_store(|store| {
    let keys = ["key0", "key1", "key2", "key3", "key4"];
    for key in keys {
      let value = store.get(key);
      assert!(value.is_none());
    }

    let pairs = keys
      .iter()
      .enumerate()
      .map(|(i, key)| (*key, i))
      .collect_vec();

    store.patch(pairs).unwrap();

    for (i, key) in keys.iter().enumerate() {
      let value = store.get(key).unwrap();
      assert_eq!(value, &Value::from(i));
    }
  })
  .await;
}

#[tokio::test]
async fn save() {
  with_store(|store| {
    assert_exists(&store.path(), false);
    store.set("key", 42).unwrap();
    store.save_now().unwrap();
    assert_exists(&store.path(), true);
  })
  .await;
}

#[tokio::test]
async fn save_debounced() {
  let (path, _permit) = with_store(|store| {
    assert_exists(&store.path(), false);
    store.save_on_change(true);
    store.set_save_strategy(SaveStrategy::debounce_millis(100));

    store.set("key", 42).unwrap();
    store.set("key2", 43).unwrap();
    store.set("key3", 44).unwrap();

    assert_exists(&store.path(), false);

    store.path()
  })
  .await;

  sleep(Duration::from_millis(200)).await;

  assert_exists(&path, true);
}

#[tokio::test]
async fn save_now() {
  with_store(|store| {
    assert_exists(&store.path(), false);
    store.save_on_change(true);
    store.set_save_strategy(SaveStrategy::debounce_millis(100));

    store.set("key", 42).unwrap();
    store.save_now().unwrap();
    assert_exists(&store.path(), true);
  })
  .await;
}

#[tokio::test]
async fn set_save_strategy() {
  with_store(|store| {
    let strategy = store.save_strategy();
    assert!(matches!(strategy, SaveStrategy::Immediate));

    store.set_save_strategy(SaveStrategy::debounce_millis(100));
    let strategy = store.save_strategy();
    assert!(matches!(strategy, SaveStrategy::Debounce(_)));

    store.set_save_strategy(SaveStrategy::throttle_millis(100));
    let strategy = store.save_strategy();
    assert!(matches!(strategy, SaveStrategy::Throttle(_)));
  })
  .await;
}

#[tokio::test]
async fn save_on_change() {
  with_store(|store| {
    assert_exists(&store.path(), false);
    store.save_on_change(true);
    store.set("key", 42).unwrap();
    assert_exists(&store.path(), true);
  })
  .await;
}

#[tokio::test]
async fn watch() {
  let notify = Arc::new(Notify::new());
  with_store(|store| {
    let notify = Arc::clone(&notify);
    store.watch(move |_| {
      notify.notify_one();
      Ok(())
    });

    store.set("key", 42).unwrap();
  })
  .await;

  timeout(Duration::from_millis(200), notify.notified())
    .await
    .unwrap();
}

#[tokio::test]
async fn unwatch() {
  with_store(|store| {
    let id = store.watch(|_| Ok(()));
    assert!(store.unwatch(id));
  })
  .await;
}

#[tokio::test]
async fn has() {
  with_store(|store| {
    assert!(!store.has("key"));
    store.set("key", 42).unwrap();
    assert!(store.has("key"));
  })
  .await;
}

#[tokio::test]
async fn keys() {
  with_store(|store| {
    assert!(store.keys().count() == 0);
    store.set("key", 42).unwrap();
    assert!(store.keys().next().unwrap() == "key");
  })
  .await;
}

#[tokio::test]
async fn values() {
  with_store(|store| {
    assert!(store.values().count() == 0);
    store.set("key", 42).unwrap();
    assert!(store.values().next().unwrap() == &Value::from(42));
  })
  .await;
}

#[tokio::test]
async fn entries() {
  with_store(|store| {
    assert!(store.entries().count() == 0);
    store.set("key", 42).unwrap();

    let (key, value) = store.entries().next().unwrap();
    assert!(key == "key" && value == &Value::from(42));
  })
  .await;
}

#[tokio::test]
async fn len() {
  with_store(|store| {
    assert!(store.len() == 0);
    store.set("key", 42).unwrap();
    assert!(store.len() == 1);
  })
  .await;
}

#[tokio::test]
async fn is_empty() {
  with_store(|store| {
    assert!(store.is_empty());
    store.set("key", 42).unwrap();
    assert!(!store.is_empty());
  })
  .await;
}

async fn with_store<'a, F, T>(f: F) -> (T, SemaphorePermit<'a>)
where
  F: FnOnce(&mut Store<MockRuntime, DefaultMarker>) -> T,
{
  let permit = acquire_permit().await.unwrap();
  let value = HANDLE
    .store_collection()
    .with_store(&*STORE_ID, f)
    .unwrap();

  (value, permit)
}

async fn acquire_permit<'a>() -> Result<SemaphorePermit<'a>> {
  let permit = SEMAPHORE.acquire().await?;
  HANDLE
    .store_collection()
    .unload_store(&STORE_ID)?;

  let temp_dir = temp_dir();
  if fs::try_exists(temp_dir).await? {
    fs::remove_dir_all(temp_dir).await?;
  }

  Ok(permit)
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
