# Changelog

## Next

### Breaking changes

- Change the default directory name for the stores (from `pinia` to `tauri-plugin-pinia`). If you're using a custom path, this won't affect you. Otherwise, you'll need to move your existing stores to the new default directory or manually set the path as it was before.

```rust
use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // The old default path.
      let path = app
        .path()
        .app_data_dir()
        .expect("failed to resolve app data dir")
        .join("pinia");

      let pinia = tauri_plugin_pinia::Builder::new()
        .path(path)
        .build();

      app
        .app_handle()
        .plugin(pinia)
        .expect("failed to add pinia plugin");

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

## 0.9.1

### Documentation

- Fix broken link in the README.

## 0.9.0

### Breaking changes

- Update [`tauri-store`](https://crates.io/crates/tauri-store) to [`0.4.0`](./tauri-store.md#040).

### Features

- Add [`saveInterval`](https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/interfaces/StoreOptions.html#saveinterval), [`saveStrategy`](https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/interfaces/StoreOptions.html#savestrategy), and [`saveOnChange`](https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/interfaces/StoreOptions.html#saveonchange) options.

### Bug fixes

- Merge options from each store with the global options.
