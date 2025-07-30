---
layout: doc
title: Changelog
titleTemplate: '@tauri-store/zustand'
description: Changelog for @tauri-store/zustand
outline: 2
---

# @tauri-store/zustand

## Next

_Nothing yet._

## 1.0.0

### Breaking Changes

- Update `tauri-store` to `1.0.0`.

## 0.3.1

### Dependencies

- Update minor dependencies.

## 0.3.0

### Features

- Allow adding or removing stores from the save and sync deny lists after the plugin has been built.
- Add [`allowSave`](https://tb.dev.br/tauri-store/js-docs/plugin-zustand/variables/allowSave.html), [`allowSync`](https://tb.dev.br/tauri-store/js-docs/plugin-zustand/variables/allowSync.html), [`denySave`](https://tb.dev.br/tauri-store/js-docs/plugin-zustand/variables/denySave.html), and [`denySync`](https://tb.dev.br/tauri-store/js-docs/plugin-zustand/variables/denySync.html) commands.
- Add [`save`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/interfaces/StoreFrontendOptions.html#save) and [`sync`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/interfaces/StoreFrontendOptions.html#sync) options.

### Bug fixes

- Stop unnecessarily requiring `filterKeys` to be set when the strategy is a callback.

## 0.2.1

### Features

- Expose marker struct.

## 0.2.0

### Features

- Allow to start stores automatically.

### Dependencies

- Update `tauri-store` to `0.11.0`.

## 0.1.2

### Features

- Expose [`TimeStrategy`](https://tb.dev.br/tauri-store/js-docs/shared/classes/TimeStrategy.html), [`TimeStrategyKind`](https://tb.dev.br/tauri-store/js-docs/shared/types/TimeStrategyKind.html), [`LooseTimeStrategyKind`](https://tb.dev.br/tauri-store/js-docs/shared/types/LooseTimeStrategyKind.html), [`StoreKeyFilter`](https://tb.dev.br/tauri-store/js-docs/shared/types/StoreKeyFilter.html), and [`StoreKeyFilterStrategy`](https://tb.dev.br/tauri-store/js-docs/shared/types/StoreKeyFilterStrategy.html) types.

## 0.1.1

### Features

- Add [`try_state_or`](https://docs.rs/tauri-plugin-zustand/0.1.1/tauri_plugin_zustand/struct.Store.html#method.try_state_or), [`try_state_or_default`](https://docs.rs/tauri-plugin-zustand/0.1.1/tauri_plugin_zustand/struct.Store.html#method.try_state_or_default), and [`try_state_or_else`](https://docs.rs/tauri-plugin-zustand/0.1.1/tauri_plugin_zustand/struct.Store.html#method.try_state_or_else) methods.
