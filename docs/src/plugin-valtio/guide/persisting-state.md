---
layout: doc
title: Persisting state
titleTemplate: '@tauri-store/valtio'
description: Persisting store state
---

# Persisting state

All your stores are automatically persisted to disk upon a graceful exit, but you can also manually save them if needed:

::: code-group

```typescript [JavaScript]
import { save, saveAll } from '@tauri-store/valtio';

// Save a single store.
await save('my-store');

// Save some stores.
await save('store-1', 'store-2', 'store-3');

// Save all stores.
await saveAll();
```

```rust [Rust]
use tauri_plugin_valtio::ManagerExt;

// Here, "manager" represents any type that implements the "Manager" trait provided by Tauri.
// This includes "AppHandle", "Window", and "WebviewWindow".
// See: https://docs.rs/tauri/latest/tauri/trait.Manager.html

// Save a single store.
manager.valtio().save("my-store");

// Save some stores.
manager.valtio().save_some(&["store-1", "store-2"]);

// Save all stores.
manager.valtio().save_all();
```

:::

## Save on change

If there's a need to save a store whenever its state changes, you can enable the [`saveOnChange`](https://tb.dev.br/tauri-store/js-docs/plugin-valtio/interfaces/StoreBackendOptions.html#saveonchange) option when defining the store.

```typescript
import { store } from '@tauri-store/valtio';

const value = { counter: 0 };
const counterStore = store('counter', value, {
  saveOnChange: true,

  // You can also debounce or throttle when saving.
  // This is optional. The default behavior is to save immediately.
  saveStrategy: 'debounce',
  saveInterval: 1000,
});
```

## Autosave

You can also enable [`autosave`](https://docs.rs/tauri-plugin-valtio/latest/tauri_plugin_valtio/struct.Builder.html#method.autosave) to periodically write the stores to disk.

::: code-group

```rust [src-tauri/src/lib.rs]
use std::time::Duration;

// Save every five minutes.
tauri_plugin_valtio::Builder::new()
  .autosave(Duration::from_secs(300))
  .build();
```

:::

## Custom directory

By default, the stores are saved in a directory called `tauri-plugin-valtio` within your app's [data directory](https://docs.rs/tauri/latest/tauri/path/struct.PathResolver.html#method.app_data_dir). You can change this by setting the [`path`](https://docs.rs/tauri-plugin-valtio/latest/tauri_plugin_valtio/struct.Builder.html#method.path) option during the plugin's initialization.

::: code-group

```rust [src-tauri/src/lib.rs]
tauri_plugin_valtio::Builder::new()
  .path("/path/to/custom/directory")
  .build();
```

:::

The path can also be modified at runtime. In this case, all _currently active_ stores will be moved to the new directory.

::: code-group

```typescript [JavaScript]
import { setStoreCollectionPath } from '@tauri-store/valtio';

await setStoreCollectionPath('/path/to/new/directory');
```

```rust [Rust]
use tauri_plugin_valtio::ManagerExt;

manager.valtio().set_path("/path/to/new/directory");
```

:::

## Denylist

If a store should be [synchronized](./synchronization.md), but not saved to disk, you can add it to the [denylist](https://docs.rs/tauri-plugin-valtio/latest/tauri_plugin_valtio/struct.Builder.html#method.save_denylist).

::: code-group

```rust [src-tauri/src/lib.rs]
tauri_plugin_valtio::Builder::new()
  .save_denylist(&["store-1", "store-2"])
  .build();
```

:::
