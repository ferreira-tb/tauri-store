# Accessing from Rust

When the [`ManagerExt`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/trait.ManagerExt.html) trait is in scope, you can access your stores from any type that implements the [Manager](https://docs.rs/tauri/latest/tauri/trait.Manager.html) trait (e.g. [`AppHandle`](https://docs.rs/tauri/latest/tauri/struct.AppHandle.html), [`Window`](https://docs.rs/tauri/latest/tauri/window/struct.Window.html), [`WebviewWindow`](https://docs.rs/tauri/latest/tauri/window/struct.Window.html)).

Note that all values are stored as [`serde_json::Value`](https://docs.rs/serde_json/latest/serde_json/enum.Value.html), so you may need to convert them to the desired type when accessing from Rust. You can check the [serde_json documentation](https://docs.rs/serde_json/latest/serde_json/) for more information.

A list of all available methods for the stores can be found [here](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.Store.html).

```rust
use tauri_plugin_pinia::ManagerExt;

#[tauri::command]
async fn get_counter(app: AppHandle) -> Option<i32> {
  app
    .pinia()
    .get("store", "counter")
    .and_then(|counter| serde_json::from_value(counter).ok())
}
```

You can also use the [`try_get`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.Pinia.html#method.try_get) method to get the value directly as the desired type.

```rust
#[tauri::command]
async fn try_get_counter(app: AppHandle) -> i32 {
  app
    .pinia()
    .try_get::<i32>("store", "counter")
    .unwrap()
}
```
