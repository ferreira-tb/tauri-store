# Changelog

## Next

_Nothing yet._

## 0.10.0 {#v0-10-0}

### Breaking changes

- Update [`tauri-store`](https://docs.rs/tauri-store/0.5.0/tauri_store/) to [`0.5.0`](./tauri-store.md#v0-5-0).
- Change the default directory name for stores from `pinia` to `tauri-plugin-pinia`. If you’re using a [custom path](https://docs.rs/tauri-plugin-pinia/0.10.0/tauri_plugin_pinia/struct.Builder.html#method.path), this change won’t affect you. Otherwise, you’ll need to either move your existing stores to the new default directory or manually set the path to match the previous configuration.

```rust
use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // The old default path.
      let path = app.path().app_data_dir()?.join("pinia");
      app.app_handle().plugin(
        tauri_plugin_pinia::Builder::new()
          .path(path)
          .build(),
      )?;

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

## 0.9.1 {#v0-9-1}

### Documentation

- Fix broken link in the README.

## 0.9.0 {#v0-9-0}

### Breaking changes

- Update [`tauri-store`](https://docs.rs/tauri-store/0.4.0/tauri_store/) to [`0.4.0`](./tauri-store.md#v0-4-0).

### Features

- Add [`saveInterval`](https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/interfaces/StoreOptions.html#saveinterval), [`saveStrategy`](https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/interfaces/StoreOptions.html#savestrategy), and [`saveOnChange`](https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/interfaces/StoreOptions.html#saveonchange) options.

### Bug fixes

- Merge options from each store with the global options.
