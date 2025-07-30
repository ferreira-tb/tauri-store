---
layout: doc
title: Changelog
description: Changelog for tauri-store
outline: 2
---

# tauri-store

## Next

_Nothing yet._

## 1.0.0

### Breaking Changes

- Rename `Store::get` to `Store::get_raw`.
- Rename `Store::try_get`, `Store::try_get_or`, `Store::try_get_or_default`, and `Store::try_get_or_else` to `Store::get`, `Store::get_or`, `Store::get_or_default`, and `Store::get_or_else`, respectively.
- Rename `Store::state` to `Store::raw_state`.
- Rename `StoreCollection::try_state`, `StoreCollection::try_state_or`, `StoreCollection::try_state_or_default`, and `StoreCollection::try_state_or_else` to `StoreCollection::state`, `StoreCollection::state_or`, `StoreCollection::state_or_default`, and `StoreCollection::state_or_else`, respectively.

## 0.12.1

### Dependencies

- Update minor dependencies.

## 0.12.0

### Features

- Allow adding or removing stores from the save and sync deny lists after the plugin has been built.
- Add [`allowSave`](https://tb.dev.br/tauri-store/js-docs/tauri-store/variables/allowSave.html), [`allowSync`](https://tb.dev.br/tauri-store/js-docs/tauri-store/variables/allowSync.html), [`denySave`](https://tb.dev.br/tauri-store/js-docs/tauri-store/variables/denySave.html), and [`denySync`](https://tb.dev.br/tauri-store/js-docs/tauri-store/variables/denySync.html) commands.
- Add [`save`](https://tb.dev.br/tauri-store/js-docs/tauri-store/interfaces/StoreFrontendOptions.html#save) and [`sync`](https://tb.dev.br/tauri-store/js-docs/tauri-store/interfaces/StoreFrontendOptions.html#sync) options.

### Bug fixes

- Stop unnecessarily requiring `filterKeys` to be set when the strategy is a callback.

## 0.11.0

### Breaking Changes

- Introduce a [`CollectionMarker`](https://docs.rs/tauri-store/0.11.0/tauri_store/trait.CollectionMarker.html) generic for the [`Store`](https://docs.rs/tauri-store/0.11.0/tauri_store/struct.Store.html) and [`StoreCollection`](https://docs.rs/tauri-store/0.11.0/tauri_store/struct.StoreCollection.html) types to enable plugin interoperability.
- Remove top-level [`with_store`](https://docs.rs/tauri-store/0.10.5/tauri_store/fn.with_store.html) helper.

### Features

- Allow to start stores automatically.
- Add [`store_collection_with_marker`](https://docs.rs/tauri-store/0.11.0/tauri_store/trait.ManagerExt.html#method.store_collection_with_marker).

## 0.10.5

### Features

- Expose [`TimeStrategy`](https://tb.dev.br/tauri-store/js-docs/shared/classes/TimeStrategy.html), [`TimeStrategyKind`](https://tb.dev.br/tauri-store/js-docs/shared/types/TimeStrategyKind.html), [`LooseTimeStrategyKind`](https://tb.dev.br/tauri-store/js-docs/shared/types/LooseTimeStrategyKind.html), [`StoreKeyFilter`](https://tb.dev.br/tauri-store/js-docs/shared/types/StoreKeyFilter.html), and [`StoreKeyFilterStrategy`](https://tb.dev.br/tauri-store/js-docs/shared/types/StoreKeyFilterStrategy.html) types.

## 0.10.4

### Dependencies

- Update dependencies.

## 0.10.3

### Features

- Add [`try_state_or`](https://docs.rs/tauri-store/0.10.3/tauri_store/struct.Store.html#method.try_state_or), [`try_state_or_default`](https://docs.rs/tauri-store/0.10.3/tauri_store/struct.Store.html#method.try_state_or_default), and [`try_state_or_else`](https://docs.rs/tauri-store/0.10.3/tauri_store/struct.Store.html#method.try_state_or_else) methods.

## 0.10.2

### Dependencies

- Update dependencies.

## 0.10.1

### Documentation

- Update README.

## 0.10.0

### Breaking Changes

- Remove `Store.subscribeKey`.

### Features

- Add experimental support for store migrations.
- Use [`alien-signals`](https://www.npmjs.com/package/alien-signals) as the underlying reactivity system.

## 0.9.0

### Breaking Changes

- Use different files for the stores when in development mode.
- Add [`StoreId`](https://docs.rs/tauri-store/0.9.0/tauri_store/struct.StoreId.html) struct.
- Add [`WatcherId`](https://docs.rs/tauri-store/0.9.0/tauri_store/struct.WatcherId.html) struct.
- Remove deprecated `StoreFrontendOptions.onError`.

## 0.8.1

### Documentation

- Ensure that Cargo features are properly documented on [`docs.rs`](https://docs.rs/tauri-store/0.8.1/tauri_store).

## 0.8.0

### Breaking Changes

- Add `plugin` feature. Now it's possible use `tauri-store` directly as a Tauri plugin. This is enabled by default, so anyone developing a plugin based on the `tauri-store` crate should make sure to disable this feature.

## 0.7.2

### Documentation

- Update README.

## 0.7.1

### Bug Fixes

- Remove `#[expect]` attribute.

## 0.7.0

### Features

- Add [`StoreCollection::set_path`](https://docs.rs/tauri-store/0.7.0/tauri_store/struct.StoreCollection.html#method.set_path).
- Add [`Store::save_on_exit`](https://docs.rs/tauri-store/0.7.0/tauri_store/struct.Store.html#method.save_on_exit) and [`StoreOptions::save_on_exit`](https://docs.rs/tauri-store/0.7.0/tauri_store/struct.StoreOptions.html#structfield.save_on_exit).

## 0.6.0

### Breaking Changes

- [`StoreState`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreState.html) is now a struct instead of a type alias.
- Remove the `StoreStateExt` trait.
- Rename `StoreCollection::store_state` to [`StoreCollection::state`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.state).
- Rename `StoreCollection::try_store_state` to [`StoreCollection::try_state`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.try_state).

### Features

- Add [`Store::try_get_or`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.try_get_or), [`Store::try_get_or_default`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.try_get_or_default), and [`Store::try_get_or_else`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.try_get_or_else).
- Add [`StoreCollection::try_get_or`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.try_get_or), [`StoreCollection::try_get_or_default`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.try_get_or_default), and [`StoreCollection::try_get_or_else`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.try_get_or_else).
- Add [`ManagerExt::with_store`](https://docs.rs/tauri-store/0.6.0/tauri_store/trait.ManagerExt.html#method.with_store) as a provided method.

### Enhancements

- [`Store::patch`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.patch), [`Store::patch_with_source`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.patch_with_source), [`Store::set`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.set), [`StoreCollection::patch`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.patch), and [`StoreCollection::set`](https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.set) now take a generic parameter for the value type.

## 0.5.0

### Breaking Changes

- Remove `unstable-async` feature.

### Enhancements

- Add `prelude` module.

## 0.4.0

### Breaking Changes

- Take `&mut self` in [`Store::watch`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.watch) and [`Store::unwatch`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.unwatch).
- Return `T` instead of `Result<T, Error>` from [`with_store`](https://docs.rs/tauri-store/0.4.0/tauri_store/fn.with_store.html) functions.
- [`Store::set`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.set), [`Store::patch`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.patch), and [`Store::patch_with_source`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.patch_with_source) are now async.
- Remove `ahash` feature.
- Remove `boxed` and `boxed_ok` macros.

### Features

- Add [`StoreCollection::default_save_strategy`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.default_save_strategy) and [`StoreCollectionBuilder::default_save_strategy`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollectionBuilder.html#method.default_save_strategy).
- Add [`StoreCollection::save_now`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.save_now), [`StoreCollection::save_some_now`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.save_some_now), and [`StoreCollection::save_all_now`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.save_all_now), to save a store immediately, ignoring the save strategy.
- Add [`Store::save_on_change`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.save_on_change).
- Add [`Store::save_now`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.save_now).
- Add [`Store::set_options`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.set_options).
- Add [`Store::save_strategy`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.save_strategy) and [`Store::set_save_strategy`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.set_save_strategy).
- Allow debouncing and throttling when saving the stores.
- Add `boxed` function.

### Enhancements

- [`StoreCollectionBuilder::autosave`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollectionBuilder.html#method.autosave), [`StoreCollection::clear_autosave`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.clear_autosave), and [`StoreCollection::set_autosave`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.set_autosave) are no longer gated by the `unstable-async` feature.

### Bug Fixes

- Consume the first autosave tick immediately before starting the interval.

### Performance

- Use the [`ResourceTable`](https://docs.rs/tauri/2.1.1/tauri/struct.ResourceTable.html) to manage each store independently, instead of using a single hash map for all of them.
