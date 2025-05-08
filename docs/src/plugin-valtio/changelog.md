---
layout: doc
title: Changelog
titleTemplate: '@tauri-store/valtio'
description: Changelog for @tauri-store/valtio
outline: 2
---

# @tauri-store/valtio

## Next

_Nothing yet._

## 2.4.0

### Features

- Expose [`TimeStrategyKind`](https://tb.dev.br/tauri-store/js-docs/shared/types/TimeStrategyKind.html) and [`LooseTimeStrategyKind`](https://tb.dev.br/tauri-store/js-docs/shared/types/LooseTimeStrategyKind.html) types.

## 2.3.0

### Features

- Add [`try_state_or`](https://docs.rs/tauri-plugin-valtio/2.3.0/tauri_plugin_valtio/struct.Store.html#method.try_state_or), [`try_state_or_default`](https://docs.rs/tauri-plugin-valtio/2.3.0/tauri_plugin_valtio/struct.Store.html#method.try_state_or_default), and [`try_state_or_else`](https://docs.rs/tauri-plugin-valtio/2.3.0/tauri_plugin_valtio/struct.Store.html#method.try_state_or_else) methods.

## 2.2.0

### Features

- Add `toStore` helper function to create a store from an existing proxy.

```typescript
import { proxy } from 'valtio';
import { toStore } from '@tauri-store/valtio';

const state = proxy({ counter: 0 });
const foo = toStore('foo', state);
```

## 2.1.1

### Documentation

- Update README.

## 2.1.0

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
