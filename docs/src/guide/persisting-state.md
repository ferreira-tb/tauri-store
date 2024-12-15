<script setup lang="ts">
import { useSelectedPlugin } from '@/composables/plugin';

const plugin = useSelectedPlugin();
</script>

# Persisting state

All your stores are automatically persisted to disk upon graceful exit, so usually there's no need to manually save them. However, if you want to do so:

```ts-vue
import { save, saveAll } from '{{ plugin.name }}';

// Save a single store.
await save('my-store');

// Save some stores.
await save('my-store', 'my-store-2');

// Save all stores.
await saveAll();
```

## Save on change

If there's a need to save a store whenever its state changes, you can enable the <DocsTs path="interfaces/StoreOptions.html#saveonchange">`saveOnChange`</DocsTs> option when defining the store.

<div class="tauri-plugin-pinia">

<!--@include: ../examples/save-on-change/pinia.md-->

</div>

## Autosave

You can also enable <DocsRs path="struct.Builder.html#method.autosave">`autosave`</DocsRs> to periodically write the stores to disk.

```rust-vue{6}
use std::time::Duration;

tauri::Builder::default()
  .plugin(
    {{ plugin.snakeName }}::Builder::new()
      .autosave(Duration::from_secs(300))
      .build(),
  )
  .run(tauri::generate_context!())
  .expect("error while running tauri application");

```

## Custom directory

By default, the stores are saved in the [app's data directory](https://docs.rs/tauri/latest/tauri/path/struct.PathResolver.html#method.app_data_dir). You can change this by setting the [`path`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.Builder.html#method.path) option when initializing the plugin.

```rust-vue{4} [Rust]
tauri::Builder::default()
  .plugin(
    {{ plugin.snakeName }}::Builder::new()
      .path("/path/to/custom/directory")
      .build(),
  )
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```
