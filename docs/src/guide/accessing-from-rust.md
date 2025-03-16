---
layout: doc
title: Accessing from Rust
description: Accessing stores from Rust
---

# Accessing from Rust

When the [`ManagerExt`](https://docs.rs/tauri-store/latest/tauri_store/trait.ManagerExt.html) trait is in scope, you can access your stores from any type that implements the [`Manager`](https://docs.rs/tauri/latest/tauri/trait.Manager.html) trait (e.g. [`AppHandle`](https://docs.rs/tauri/latest/tauri/struct.AppHandle.html), [`Window`](https://docs.rs/tauri/latest/tauri/window/struct.Window.html), [`WebviewWindow`](https://docs.rs/tauri/latest/tauri/window/struct.Window.html)).

Note that all values are stored as [`serde_json::Value`](https://docs.rs/serde_json/latest/serde_json/enum.Value.html), so you may need to convert them to the desired type when accessing from Rust. You can check the [`serde_json` documentation](https://docs.rs/serde_json/latest/serde_json/) for more information.

A list of all available methods for the stores can be found [here](https://docs.rs/tauri-store/latest/tauri_store/struct.Store.html).

```rust
use tauri_store::ManagerExt;

#[tauri::command]
fn get_counter(app: AppHandle) -> i32 {
  let value = app
    .store_collection()
    .get("store", "counter")
    .unwrap();

  serde_json::from_value(value).unwrap()
}
```

You can also use the [`try_get`](https://docs.rs/tauri-store/latest/tauri_store/struct.StoreCollection.html#method.try_get) method to get the value directly as the desired type.

```rust
use tauri_store::ManagerExt;

#[tauri::command]
fn try_get_counter(app: AppHandle) -> i32 {
  app
    .store_collection()
    .try_get::<i32>("store", "counter")
    .unwrap()
}
```

## Watching for changes

The [`watch`](https://docs.rs/tauri-store/latest/tauri_store/struct.StoreCollection.html#method.watch) method can be used to set up a closure that will be called whenever the state of the store changes.

```rust
use tauri_store::ManagerExt;

#[tauri::command]
fn watch_store(app: AppHandle) {
  let id = app.store_collection().watch("store", |app| {
    let counter = app
      .store_collection()
      .try_get::<i32>("store", "counter")?;

    println!("counter: {counter}");

    Ok(())
  });

  // It returns an id that can be used to remove the watcher.
  if let Ok(id) = id {
    app.store_collection().unwatch("store", id).unwrap();
  }
}
```
