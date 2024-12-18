# Changelog

## Next

_Nothing yet._

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
