# tauri-plugin-pinia

Persistent Pinia stores for Tauri, with automatic synchronization between multiple windows.

## Install

Install the plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-pinia = 0.1
```

## Usage

Enable the required permissions in your capabilities file:

```json
{
  "permissions": ["event:allow-listen", "event:allow-unlisten", "pinia:default"]
}
```

Register the plugin with Tauri:

`src-tauri/src/main.rs`

```rust
fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_pinia::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

Enable the plugin for Pinia:

`src/index.ts`

```ts
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { TauriPluginPinia } from 'tauri-plugin-pinia';

const app = createApp(App);

const pinia = createPinia();
pinia.use(TauriPluginPinia);

app.use(pinia);
app.mount('#app');
```

Create your Pinia store:

`src/stores/counter.ts`

```ts
import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useCounterStore = defineStore('counter', () => {
  const counter = ref(0);

  function increment() {
    counter.value++;
  }

  function decrement() {
    counter.value--;
  }

  return {
    counter,
    increment,
    decrement
  };
});
```
