---
layout: doc
title: Changelog
titleTemplate: '@tauri-store/pinia'
description: Changelog for @tauri-store/pinia
outline: 2
---

# @tauri-store/pinia

## Next

- Update `tauri-store` to `0.9.1`.

## 3.0.0

### Breaking Changes

- Update [`tauri-store`](https://docs.rs/tauri-store/0.9.0/tauri_store/) to `0.9.0`.
- Rename `getPiniaPath` to `getStoreCollectionPath`.
- Rename `setPiniaPath` to `setStoreCollectionPath`.
- Remove deprecated `StoreFrontendOptions.onError`.

## 2.0.0

### Breaking Changes

- Update [`pinia`](https://github.com/vuejs/pinia/releases/tag/v3.0.0) to `3.0`.

## 1.2.1

### Dependencies

- Update [`tauri-store`](https://docs.rs/tauri-store/0.8.0/tauri_store/) to `0.8.0`.

## 1.2.0

### Features

- Deprecate `StoreFrontendOptions.onError`.
- Add `StoreHooks` to the plugin options.

## 1.1.2

### Dependencies

- Update [`tauri-store`](https://docs.rs/tauri-store/0.7.1/tauri_store/) to `0.7.1`.

## 1.1.1

### Dependencies

- Update [`@tauri-store/shared`](https://www.npmjs.com/package/@tauri-store/shared/v/0.4.1) to `0.4.1`.

## 1.1.0

### Features

- Add `setPiniaPath`.
- Add `StoreBackendOptions.saveOnExit`.

### Dependencies

- Update [`tauri-store`](https://docs.rs/tauri-store/0.7.0/tauri_store/) to `0.7.0`.

## 1.0.0

### Breaking Changes

- Update [`tauri-store`](https://docs.rs/tauri-store/0.6.0/tauri_store/) to `0.6.0`.

## 0.10.3

### Dependencies

- Update [`@tauri-store/shared`](https://www.npmjs.com/package/@tauri-store/shared).

## 0.10.2

### Bug Fixes

- Fix the filter for store keys.

## 0.10.1

### Features

- Expose `State` type.

## 0.10.0

### Breaking Changes

- Update [`tauri-store`](https://docs.rs/tauri-store/0.5.0/tauri_store/) to `0.5.0`.
- Change the default directory name for stores from `pinia` to `tauri-plugin-pinia`.

## 0.9.1

### Bug Fixes

- Fix broken link in the README.

## 0.9.0

### Breaking Changes

- Update [`tauri-store`](https://docs.rs/tauri-store/0.4.0/tauri_store/) to `0.4.0`.

### Features

- Add `saveInterval`, `saveStrategy`, and `saveOnChange` options.

### Bug Fixes

- Merge options from each store with the global options.
