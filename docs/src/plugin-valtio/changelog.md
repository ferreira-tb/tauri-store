---
layout: doc
title: Changelog
titleTemplate: '@tauri-store/valtio'
description: Changelog for @tauri-store/valtio
outline: 2
---

# @tauri-store/valtio

## Next

### Features

- Add experimental support for store migrations.

### Bug Fixes

- Fix panic when calling [`ManagerExt::valtio`](https://docs.rs/tauri-plugin-valtio/2.0.0/tauri_plugin_valtio/trait.ManagerExt.html#method.valtio) from [`Window`](https://docs.rs/tauri/2.3.1/tauri/window/struct.Window.html) or [`WebviewWindow`](https://docs.rs/tauri/2.3.1/tauri/webview/struct.WebviewWindow.html).

### Dependencies

- Update `tauri-store` to `0.10.0`.

## 2.0.0

### Breaking Changes

- Update [`tauri-store`](https://docs.rs/tauri-store/0.9.0/tauri_store/) to `0.9.0`.
- Rename `getValtioPath` to `getStoreCollectionPath`.
- Rename `setValtioPath` to `setStoreCollectionPath`.
- Remove deprecated `StoreFrontendOptions.onError`.

## 1.1.1

### Dependencies

- Update [`tauri-store`](https://docs.rs/tauri-store/0.8.0/tauri_store/) to `0.8.0`.

## 1.1.0

### Features

- Deprecate `StoreFrontendOptions.onError`.
- Add `StoreHooks` to the plugin options.

## 0.2.2

### Dependencies

- Update [`tauri-store`](https://docs.rs/tauri-store/0.7.1/tauri_store/) to `0.7.1`.

## 0.2.1

### Dependencies

- Update [`@tauri-store/shared`](https://www.npmjs.com/package/@tauri-store/shared/v/0.4.1) to `0.4.1`.

## 0.2.0

### Features

- Add `setValtioPath`.
- Add `StoreBackendOptions.saveOnExit`.

### Dependencies

- Update [`tauri-store`](https://docs.rs/tauri-store/0.7.0/tauri_store/) to `0.7.0`.
