mod shared;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::Value;
use shared::{CONTEXT, STORE_ID};
use std::sync::Arc;
use tauri_store::SaveStrategy;
use tokio::sync::Notify;
use tokio::time::{sleep, timeout, Duration};

#[derive(Deserialize)]
struct Foo {
  key: u8,
}

#[tokio::test]
async fn id() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    assert_eq!(store.id(), STORE_ID);
  });
}

#[tokio::test]
async fn try_state() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let state = store.try_state::<Foo>();
    assert!(state.is_err());

    store.set("key", 42).unwrap();
    let state = store.try_state::<Foo>();
    assert!(state.is_ok_and(|state| state.key == 42));
  });
}

#[tokio::test]
async fn get() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let value = store.get("key");
    assert!(value.is_none());

    store.set("key", "value").unwrap();
    let value = store.get("key").unwrap();
    assert_eq!(value, &Value::from("value"));
  });
}

#[tokio::test]
async fn try_get() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let value = store.try_get::<u8>("key");
    assert!(value.is_err());

    store.set("key", 42).unwrap();
    let value = store.try_get::<u8>("key");
    assert_eq!(value.unwrap(), 42);
  });
}

#[tokio::test]
async fn try_get_or() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let value = store.try_get_or::<u8>("key", 20);
    assert_eq!(value, 20);

    store.set("key", 42).unwrap();
    let value = store.try_get_or::<u8>("key", 20);
    assert_eq!(value, 42);
  });
}

#[tokio::test]
async fn try_get_or_default() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let value = store.try_get_or_default::<u8>("key");
    assert_eq!(value, 0);

    store.set("key", 42).unwrap();
    let value = store.try_get_or_default::<u8>("key");
    assert_eq!(value, 42);
  });
}

#[tokio::test]
async fn try_get_or_else() {
  let _permit = CONTEXT.acquire_permit().await;

  let else_fn = || 20;
  CONTEXT.with_store(|store| {
    let value = store.try_get_or_else::<u8, _>("key", else_fn);
    assert_eq!(value, 20);

    store.set("key", 42).unwrap();
    let value = store.try_get_or_else::<u8, _>("key", else_fn);
    assert_eq!(value, 42);
  });
}

#[tokio::test]
async fn set() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let value = store.get("key");
    assert!(value.is_none());

    store.set("key", 42).unwrap();
    let value = store.get("key").unwrap();
    assert_eq!(value, &Value::from(42));
  });
}

#[tokio::test]
async fn patch() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let value = store.get("key");
    assert!(value.is_none());

    store.patch(("key", 42)).unwrap();
    let value = store.get("key").unwrap();
    assert_eq!(value, &Value::from(42));
  });
}

#[tokio::test]
async fn patch_many() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
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
  });
}

#[tokio::test]
async fn save() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let path = store.path();
    assert!(matches!(path.try_exists(), Ok(false)));

    store.set("key", 42).unwrap();
    store.save_now().unwrap();

    assert!(matches!(path.try_exists(), Ok(true)));
  });
}

#[tokio::test]
async fn save_debounced() {
  let _permit = CONTEXT.acquire_permit().await;

  let path = CONTEXT.with_store(|store| {
    let path = store.path();
    assert!(matches!(path.try_exists(), Ok(false)));

    store.save_on_change(true);
    store.set_save_strategy(SaveStrategy::debounce_millis(100));

    store.set("key", 42).unwrap();
    store.set("key2", 43).unwrap();
    store.set("key3", 44).unwrap();

    assert!(matches!(path.try_exists(), Ok(false)));

    path
  });

  sleep(Duration::from_millis(200)).await;

  assert!(matches!(path.try_exists(), Ok(true)));
}

#[tokio::test]
async fn save_now() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let path = store.path();
    assert!(matches!(path.try_exists(), Ok(false)));

    store.save_on_change(true);
    store.set_save_strategy(SaveStrategy::debounce_millis(100));

    store.set("key", 42).unwrap();
    store.save_now().unwrap();

    assert!(matches!(path.try_exists(), Ok(true)));
  });
}

#[tokio::test]
async fn set_save_strategy() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let strategy = store.save_strategy();
    assert!(matches!(strategy, SaveStrategy::Immediate));

    store.set_save_strategy(SaveStrategy::debounce_millis(100));
    let strategy = store.save_strategy();
    assert!(matches!(strategy, SaveStrategy::Debounce(_)));

    store.set_save_strategy(SaveStrategy::throttle_millis(100));
    let strategy = store.save_strategy();
    assert!(matches!(strategy, SaveStrategy::Throttle(_)));
  });
}

#[tokio::test]
async fn save_on_change() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let path = store.path();
    assert!(matches!(path.try_exists(), Ok(false)));

    store.save_on_change(true);
    store.set("key", 42).unwrap();

    assert!(matches!(path.try_exists(), Ok(true)));
  });
}

#[tokio::test]
async fn watch() {
  let _permit = CONTEXT.acquire_permit().await;

  let notify = Arc::new(Notify::new());
  CONTEXT.with_store(|store| {
    let notify = Arc::clone(&notify);
    store.watch(move |_| {
      notify.notify_one();
      Ok(())
    });

    store.set("key", 42).unwrap();
  });

  timeout(Duration::from_millis(200), notify.notified())
    .await
    .unwrap();
}

#[tokio::test]
async fn unwatch() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    let id = store.watch(|_| Ok(()));
    assert!(store.unwatch(id));
  });
}

#[tokio::test]
async fn has() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    assert!(!store.has("key"));
    store.set("key", 42).unwrap();
    assert!(store.has("key"));
  });
}

#[tokio::test]
async fn keys() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    assert!(store.keys().count() == 0);
    store.set("key", 42).unwrap();
    assert!(store.keys().next().unwrap() == "key");
  });
}

#[tokio::test]
async fn values() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    assert!(store.values().count() == 0);
    store.set("key", 42).unwrap();
    assert!(store.values().next().unwrap() == &Value::from(42));
  });
}

#[tokio::test]
async fn entries() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    assert!(store.entries().count() == 0);
    store.set("key", 42).unwrap();

    let (key, value) = store.entries().next().unwrap();
    assert!(key == "key" && value == &Value::from(42));
  });
}

#[tokio::test]
async fn len() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    assert!(store.len() == 0);
    store.set("key", 42).unwrap();
    assert!(store.len() == 1);
  });
}

#[tokio::test]
async fn is_empty() {
  let _permit = CONTEXT.acquire_permit().await;

  CONTEXT.with_store(|store| {
    assert!(store.is_empty());
    store.set("key", 42).unwrap();
    assert!(!store.is_empty());
  });
}
