# Changelog

## Next

_Nothing yet._

## 0.4.0

### Breaking changes

- Take `&mut self` in `Store::watch` and `Store::unwatch`.
- Return `T` instead of `Result<T, Error>` from the `with_store` functions.
- Remove `ahash` feature.
- <Feature name="unstable-async" /> Remove `boxed` and `boxed_ok` macros.
- <Feature name="unstable-async" /> `Store::set`, `Store::patch`, and `Store::patch_with_source` are now async.

### Features

- Add `StoreCollection::default_save_strategy`.
- Add `StoreCollection::save_now`, `StoreCollection::save_some_now`, and `StoreCollection::save_all_now`, to save the store immediately, ignoring the save strategy.
- Add `StoreCollectionBuilder::default_save_strategy`.
- Add `Store::save_on_change`.
- Add `Store::save_now`, `Store::same_some_now`, and `Store::save_all_now`.
- Add `Store::set_options`.
- Add `Store::save_strategy` and `Store::set_save_strategy`.
- Allow debouncing and throttling when saving the stores.
- <Feature name="unstable-async" /> Add `boxed` function.

### Enhancements

- `StoreCollectionBuilder::autosave`, `StoreCollection::clear_autosave`, and `StoreCollection::set_autosave` are no longer gated by the `unstable-async` feature.

### Bug fixes

- Consume the first autosave tick immediately, before starting the interval.

### Performance

- Use the [`ResourceTable`](https://docs.rs/tauri/latest/tauri/struct.ResourceTable.html#) to manage each store independently, instead of using a single hash map protected by a Mutex for all of them.
