# Changelog

## Next

_Nothing yet._

## 0.4.0

### Breaking changes

- Take `&mut self` in [`Store::watch`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.watch) and [`Store::unwatch`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.unwatch).
- Return `T` instead of `Result<T, Error>` from [`with_store`](https://docs.rs/tauri-store/0.4.0/tauri_store/fn.with_store.html) functions.
- Remove `ahash` feature.
- <Feature name="unstable-async" /> Remove `boxed` and `boxed_ok` macros.
- <Feature name="unstable-async" /> [`Store::set`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.set), [`Store::patch`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.patch), and [`Store::patch_with_source`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.patch_with_source) are now async.

### Features

- Add [`StoreCollection::default_save_strategy`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.default_save_strategy).
- Add [`StoreCollection::save_now`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.save_now), [`StoreCollection::save_some_now`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.save_some_now), and [`StoreCollection::save_all_now`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.save_all_now), to save the store immediately, ignoring the save strategy.
- Add [`StoreCollectionBuilder::default_save_strategy`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollectionBuilder.html#method.default_save_strategy).
- Add [`Store::save_on_change`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.save_on_change).
- Add [`Store::save_now`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.save_now).
- Add [`Store::set_options`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.set_options).
- Add [`Store::save_strategy`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.save_strategy) and [`Store::set_save_strategy`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.Store.html#method.set_save_strategy).
- Allow debouncing and throttling when saving the stores.
- <Feature name="unstable-async" /> Add `boxed` function.

### Enhancements

- [`StoreCollectionBuilder::autosave`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollectionBuilder.html#method.autosave), [`StoreCollection::clear_autosave`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.clear_autosave), and [`StoreCollection::set_autosave`](https://docs.rs/tauri-store/0.4.0/tauri_store/struct.StoreCollection.html#method.set_autosave) are no longer gated by the `unstable-async` feature.

### Bug fixes

- Consume the first autosave tick immediately, before starting the interval.

### Performance

- Use the [`ResourceTable`](https://docs.rs/tauri/latest/tauri/struct.ResourceTable.html#) to manage each store independently, instead of using a single hash map for all of them.
