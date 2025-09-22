---
layout: doc
title: Accessing from Rust
titleTemplate: '@tauri-store/pinia'
description: Accessing stores from Rust
---

# Accessing from Rust

When the [`ManagerExt`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/trait.ManagerExt.html) trait is in scope, you can access your stores from any type that implements the [`Manager`](https://docs.rs/tauri/latest/tauri/trait.Manager.html) trait (e.g. [`AppHandle`](https://docs.rs/tauri/latest/tauri/struct.AppHandle.html), [`Window`](https://docs.rs/tauri/latest/tauri/window/struct.Window.html), [`WebviewWindow`](https://docs.rs/tauri/latest/tauri/window/struct.Window.html)).

A list of all available methods for the stores can be found [here](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.Store.html).

```rust{7}
use tauri_plugin_pinia::ManagerExt;

#[tauri::command]
fn get_counter(app: AppHandle) -> i32 {
  app
    .pinia()
    .get::<i32>("store", "counter")
    .unwrap()
}
```

## Watching for changes

The [`watch`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.Pinia.html#method.watch) method can be used to set up a closure that will be called whenever the state of the store changes.

```rust
use tauri_plugin_pinia::ManagerExt;

#[tauri::command]
fn watch_store(app: AppHandle) {
  app.pinia().watch("store", |app| {
    let counter = app
      .pinia()
      .get::<i32>("store", "counter")?;

    println!("counter: {counter}");

    Ok(())
  });
}
```
