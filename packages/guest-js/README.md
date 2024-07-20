# tauri-plugin-pinia

Persistent Pinia stores for Tauri.

## Features

- Saves your Pinia stores to disk on application exit (or manually, if needed).
- Synchronizes your stores across multiple windows.
- Allows debouncing store updates.

## Install

- Install the Rust crate by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-pinia = 0.1
```

- Install the JavaScript package with your preferred package manager:

```sh
pnpm add tauri-plugin-pinia
# or
npm add tauri-plugin-pinia
# or
yarn add tauri-plugin-pinia
```

## Usage

> For a working example, see the [playground](https://github.com/ferreira-tb/tauri-plugin-pinia/tree/main/packages/playground).

1. Enable the required permissions in your capabilities file:

```json
{
  "permissions": ["event:allow-listen", "event:allow-unlisten", "pinia:default"]
}
```

2. Register the plugin with Tauri:

`src-tauri/src/main.rs`

```rust
fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_pinia::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

3. Enable the plugin for Pinia:

`src/index.ts`

```ts
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia';

const app = createApp(App);

const pinia = createPinia();
pinia.use(createPlugin());

app.use(pinia);
app.mount('#app');
```

4. Create your Pinia store:

`src/stores/counter.ts`

```ts
import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useCounterStore = defineStore('counter', () => {
  const counter = ref(0);

  function increment() {
    counter.value++;
  }

  return {
    counter,
    increment
  };
});
```

5. Start the plugin!

```ts
import { useCounterStore } from './stores/counter';

const counterStore = useCounterStore();
counterStore.$tauri.start();
```
