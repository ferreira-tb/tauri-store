---
layout: doc
title: Changelog
titleTemplate: '@tauri-store/svelte'
description: Changelog for @tauri-store/svelte
outline: 2
---

# @tauri-store/svelte

## Next

### Features

- Add experimental support for store migrations.

### Bug Fixes

- Fix panic when calling [`ManagerExt::svelte`](https://docs.rs/tauri-plugin-svelte/2.0.0/tauri_plugin_svelte/trait.ManagerExt.html#method.svelte) from [`Window`](https://docs.rs/tauri/2.3.1/tauri/window/struct.Window.html) or [`WebviewWindow`](https://docs.rs/tauri/2.3.1/tauri/webview/struct.WebviewWindow.html).

### Dependencies

- Update `tauri-store` to `0.10.0`.

## 2.0.0

### Breaking Changes

- Update [`tauri-store`](https://docs.rs/tauri-store/0.9.0/tauri_store/) to `0.9.0`.
- Rename `getSveltePath` to `getStoreCollectionPath`.
- Rename `setSveltePath` to `setStoreCollectionPath`.
- Remove deprecated `StoreFrontendOptions.onError`.

## 1.2.1

### Dependencies

- Update [`tauri-store`](https://docs.rs/tauri-store/0.8.0/tauri_store/) to `0.8.0`.

## 1.2.0

### Features

- Deprecate `StoreFrontendOptions.onError`.
- Add `StoreHooks` to the plugin options.

## 1.1.1

### Dependencies

- Update [`tauri-store`](https://docs.rs/tauri-store/0.7.1/tauri_store/) to `0.7.1`.

## 1.1.0

### Features

- Support [`runes`](https://svelte.dev/docs/svelte/what-are-runes).

## 1.0.0

### Features

- Add `setSveltePath`.
- Add `StoreBackendOptions.saveOnExit`.

### Dependencies

- Update [`tauri-store`](https://docs.rs/tauri-store/0.7.0/tauri_store/) to `0.7.0`.

## 0.2.0

### Breaking Changes

- Update [`tauri-store`](https://docs.rs/tauri-store/0.6.0/tauri_store/) to `0.6.0`.

## 0.1.4

### Dependencies

- Update [`@tauri-store/shared`](https://www.npmjs.com/package/@tauri-store/shared).

## 0.1.3

### Bug Fixes

- Fix the filter for store keys.

## 0.1.2

### Bug Fixes

- Set `svelte` as an external dependency.

## 0.1.1

### Features

- Expose `State` type.
