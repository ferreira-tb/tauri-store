mod shared;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::Value;
use shared::{CONTEXT, STORE_ID};

#[derive(Deserialize)]
struct Foo {
  key: u8,
}

#[test]
fn id() {
  CONTEXT.with_store(|store| {
    assert_eq!(store.id(), STORE_ID);
  });
}

#[test]
fn try_state() {
  CONTEXT.with_store(|store| {
    let state = store.try_state::<Foo>();
    assert!(state.is_err());

    store.set("key", 42).unwrap();
    let state = store.try_state::<Foo>();
    assert!(state.is_ok_and(|state| state.key == 42));
  });
}

#[test]
fn get() {
  CONTEXT.with_store(|store| {
    let value = store.get("key");
    assert!(value.is_none());

    store.set("key", "value").unwrap();
    let value = store.get("key").unwrap();
    assert_eq!(value, &Value::from("value"));
  });
}

#[test]
fn try_get() {
  CONTEXT.with_store(|store| {
    let value = store.try_get::<u8>("key");
    assert!(value.is_err());

    store.set("key", 42).unwrap();
    let value = store.try_get::<u8>("key");
    assert_eq!(value.unwrap(), 42);
  });
}

#[test]
fn try_get_or() {
  CONTEXT.with_store(|store| {
    let value = store.try_get_or::<u8>("key", 20);
    assert_eq!(value, 20);

    store.set("key", 42).unwrap();
    let value = store.try_get_or::<u8>("key", 20);
    assert_eq!(value, 42);
  });
}

#[test]
fn try_get_or_default() {
  CONTEXT.with_store(|store| {
    let value = store.try_get_or_default::<u8>("key");
    assert_eq!(value, 0);

    store.set("key", 42).unwrap();
    let value = store.try_get_or_default::<u8>("key");
    assert_eq!(value, 42);
  });
}

#[test]
fn try_get_or_else() {
  let else_fn = || 20;
  CONTEXT.with_store(|store| {
    let value = store.try_get_or_else::<u8, _>("key", else_fn);
    assert_eq!(value, 20);

    store.set("key", 42).unwrap();
    let value = store.try_get_or_else::<u8, _>("key", else_fn);
    assert_eq!(value, 42);
  });
}

#[test]
fn set() {
  CONTEXT.with_store(|store| {
    let value = store.get("key");
    assert!(value.is_none());

    store.set("key", 42).unwrap();
    let value = store.get("key").unwrap();
    assert_eq!(value, &Value::from(42));
  });
}

#[test]
fn patch() {
  CONTEXT.with_store(|store| {
    let value = store.get("key");
    assert!(value.is_none());

    store.patch(("key", 42)).unwrap();
    let value = store.get("key").unwrap();
    assert_eq!(value, &Value::from(42));
  });
}

#[test]
fn patch_many() {
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
