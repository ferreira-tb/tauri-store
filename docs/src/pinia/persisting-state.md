# Persisting state

All your stores are automatically persisted to disk upon graceful exit, so usually there's no need to manually save them. However, if you want to do so:

::: code-group

```ts [JavaScript]
import { save, saveAll } from 'tauri-plugin-pinia';

// Save a single store.
await save('my-store');

// Save some stores.
await save('my-store', 'my-store-2');

// Save all stores.
await saveAll();
```

```rust [Rust]
use tauri_plugin_pinia::ManagerExt;

// Here, `manager` represents any type that implements the `Manager` trait provided by Tauri.
// This includes `AppHandle`, `Window`, and `WebviewWindow`.

// Save a single store.
manager.pinia().save("my-store");

// Save some stores.
manager.pinia().save_some(&["my-store", "my-store-2"]);

// Save all stores.
manager.pinia().save_all();
```

:::

## Save on change

If there's a need to save a store whenever its state changes, you can enable the [`saveOnChange`](https://docs.rs/tauri-plugin-pinia/latest) option when defining the store.

```ts [JavaScript]
import { ref } from 'vue';
import { defineStore } from 'pinia';

function store() {
  const counter = ref(0);

  return {
    counter,
  };
}

export const useStore = defineStore('store', store, {
  tauri: {
    saveOnChange: true,

    // You can also debounce or throttle when saving.
    // This is optional. The default behavior is to save immediately.
    saveStrategy: 'debounce',
    saveInterval: 1000,
  },
});
```

## Autosave

You can also enable [`autosave`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.Builder.html#method.autosave) to periodically write the stores to disk.

::: code-group

```rust{6} [src-tauri/src/main.rs]
use std::time::Duration;

tauri::Builder::default()
  .plugin(
    tauri_plugin_pinia::Builder::new()
      .autosave(Duration::from_secs(300))
      .build(),
  )
  .run(tauri::generate_context!())
  .expect("error while running tauri application");

```

:::

## Custom directory

By default, the stores are saved in the [app's data directory](https://docs.rs/tauri/latest/tauri/path/struct.PathResolver.html#method.app_data_dir). You can change this by setting the [`path`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.Builder.html#method.path) option when initializing the plugin.

::: code-group

```rust{4} [src-tauri/src/main.rs]
tauri::Builder::default()
  .plugin(
    tauri_plugin_pinia::Builder::new()
      .path("/path/to/custom/directory")
      .build(),
  )
  .run(tauri::generate_context!())
  .expect("error while running tauri application");

```

:::
