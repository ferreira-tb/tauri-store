---
layout: doc
title: Changelog
titleTemplate: '@tauri-store/pinia'
description: Changelog for @tauri-store/pinia
outline: 2
---

# @tauri-store/pinia

## Next

_Nothing yet._

## 4.0.0

### Breaking Changes

- Update `tauri-store` to `1.0.0`.

Please refer to its [changelog](https://tb.dev.br/tauri-store/changelog) for more details.

## 3.7.1

### Dependencies

- Update minor dependencies.

## 3.7.0

### Features

- Allow adding or removing stores from the save and sync deny lists after the plugin has been built.
- Add [`allowSave`](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/variables/allowSave.html), [`allowSync`](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/variables/allowSync.html), [`denySave`](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/variables/denySave.html), and [`denySync`](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/variables/denySync.html) commands.
- Add [`save`](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/interfaces/StoreFrontendOptions.html#save) and [`sync`](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/interfaces/StoreFrontendOptions.html#sync) options.

### Bug fixes

- Stop unnecessarily requiring `filterKeys` to be set when the strategy is a callback.

## 3.6.0

### Features

- Expose marker struct.

## 3.5.0

### Features

- Allow to start stores automatically.

### Dependencies

- Update `tauri-store` to `0.11.0`.

## 3.4.0

### Features

- Expose [`TimeStrategy`](https://tb.dev.br/tauri-store/js-docs/shared/classes/TimeStrategy.html), [`TimeStrategyKind`](https://tb.dev.br/tauri-store/js-docs/shared/types/TimeStrategyKind.html), [`LooseTimeStrategyKind`](https://tb.dev.br/tauri-store/js-docs/shared/types/LooseTimeStrategyKind.html), [`StoreKeyFilter`](https://tb.dev.br/tauri-store/js-docs/shared/types/StoreKeyFilter.html), and [`StoreKeyFilterStrategy`](https://tb.dev.br/tauri-store/js-docs/shared/types/StoreKeyFilterStrategy.html) types.

## 3.3.0

### Features

- Add [`try_state_or`](https://docs.rs/tauri-plugin-pinia/3.3.0/tauri_plugin_pinia/struct.Store.html#method.try_state_or), [`try_state_or_default`](https://docs.rs/tauri-plugin-pinia/3.3.0/tauri_plugin_pinia/struct.Store.html#method.try_state_or_default), and [`try_state_or_else`](https://docs.rs/tauri-plugin-pinia/3.3.0/tauri_plugin_pinia/struct.Store.html#method.try_state_or_else) methods.

## 3.2.0

### Features

- Add [more options](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/interfaces/StoreBackendOptions.html) to the [createPlugin](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/functions/createPlugin.html) function.

## 3.1.2

### Bug Fixes

- Avoid recursive call in the `beforeBackendSync` hook.

## 3.1.1

### Documentation

- Update README.

## 3.1.0

### Features

- Add experimental support for store migrations.

### Bug Fixes

- Fix panic when calling [`ManagerExt::pinia`](https://docs.rs/tauri-plugin-pinia/3.0.0/tauri_plugin_pinia/trait.ManagerExt.html#method.pinia) from [`Window`](https://docs.rs/tauri/2.3.1/tauri/window/struct.Window.html) or [`WebviewWindow`](https://docs.rs/tauri/2.3.1/tauri/webview/struct.WebviewWindow.html).

### Dependencies

- Update `tauri-store` to `0.10.0`.

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
