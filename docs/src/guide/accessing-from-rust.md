<script setup lang="ts">
import { useSelectedPlugin } from '@/composables/plugin';

const plugin = useSelectedPlugin();
</script>

# Accessing from Rust

When the <DocsRs path="trait.ManagerExt.html">`ManagerExt`</DocsRs> trait is in scope, you can access your stores from any type that implements the [Manager](https://docs.rs/tauri/latest/tauri/trait.Manager.html) trait (e.g. [`AppHandle`](https://docs.rs/tauri/latest/tauri/struct.AppHandle.html), [`Window`](https://docs.rs/tauri/latest/tauri/window/struct.Window.html), [`WebviewWindow`](https://docs.rs/tauri/latest/tauri/window/struct.Window.html)).

Note that all values are stored as [`serde_json::Value`](https://docs.rs/serde_json/latest/serde_json/enum.Value.html), so you may need to convert them to the desired type when accessing from Rust. You can check the [serde_json documentation](https://docs.rs/serde_json/latest/serde_json/) for more information.

A list of all available methods for the stores can be found <DocsRs path="struct.Store.html">here</DocsRs>.

```rust-vue
use {{ plugin.snakeName }}::ManagerExt;

#[tauri::command]
async fn get_counter(app: AppHandle) -> i32 {
  let value = app
    .{{ plugin.shortName }}()
    .get("store", "counter")
    .unwrap();

  serde_json::from_value(value).unwrap()
}
```

You can also use the <DocsRs :path="`struct.${plugin.pascalShortName}.html#method.try_get`">`try_get`</DocsRs> method to get the value directly as the desired type.

```rust-vue
#[tauri::command]
async fn try_get_counter(app: AppHandle) -> i32 {
  app
    .{{ plugin.shortName }}()
    .try_get::<i32>("store", "counter")
    .unwrap()
}
```
