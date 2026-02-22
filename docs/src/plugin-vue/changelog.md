---
layout: doc
title: Changelog
titleTemplate: '@tauri-store/vue'
description: Changelog for @tauri-store/vue
outline: 2
---

# @tauri-store/vue

## Next

_Nothing yet._

## 2.2.0

### Features

- Add `preserve-order` feature.

### Dependencies

- Update `tauri-store` to `1.2.0`.

## 2.1.1

### Internal changes

- Rename `checkAutoStart` to `tryAutoStart` to better reflect behavior (thanks [RainyNight9](https://github.com/RainyNight9)).

## 2.1.0

### Features

- Update `tauri-store` to `1.1.0`.

## 2.0.2

### Documentation

- Update README.

## 2.0.1

### Bug fixes

- Remove accidentally exposed internal command.

## 2.0.0

### Breaking Changes

- Update `tauri-store` to `1.0.0`.

Please refer to its [changelog](https://tb.dev.br/tauri-store/changelog) for more details.

## 1.0.0

### Dependencies

- Update minor dependencies.

## 0.3.0

### Features

- Allow adding or removing stores from the save and sync deny lists after the plugin has been built.
- Add [`allowSave`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/variables/allowSave.html), [`allowSync`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/variables/allowSync.html), [`denySave`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/variables/denySave.html), and [`denySync`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/variables/denySync.html) commands.
- Add [`save`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/interfaces/StoreFrontendOptions.html#save) and [`sync`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/interfaces/StoreFrontendOptions.html#sync) options.

### Bug fixes

- Stop unnecessarily requiring `filterKeys` to be set when the strategy is a callback.

## 0.2.0

### Breaking Changes

- Rename `StoreRefOptions.writeDefaults` to [`StoreRefOptions.writeDefault`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/interfaces/StoreRefOptions.html#writedefault).

### Bug fixes

- Auto start global store.

## 0.1.1

### Features

- Expose marker struct.
