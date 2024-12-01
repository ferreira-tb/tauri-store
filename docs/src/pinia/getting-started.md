# Getting started

::: info Supported Tauri Version
This plugin requires Tauri `2.0` or later.
:::

## Install

Install the Rust crate by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-pinia = 0.8
```

Install the JavaScript package with your preferred package manager:

::: code-group

```sh [npm]
npm install tauri-plugin-pinia
```

```sh [pnpm]
pnpm add tauri-plugin-pinia
```

```sh [yarn]
yarn add tauri-plugin-pinia
```

```sh [bun]
bun add tauri-plugin-pinia
```

:::

## Usage

1. Enable the required permissions in your capabilities file:

::: code-group

```json{4} [src-tauri/capabilities/pinia.json]
{
  "identifier": "pinia",
  "windows": ["*"],
  "permissions": ["pinia:default", "core:event:default"]
}
```

:::

2. Register the plugin with Tauri:

::: code-group

```rust{2} [src-tauri/src/main.rs]
tauri::Builder::default()
  .plugin(tauri_plugin_pinia::init())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");

```

:::

3. Enable the plugin for Pinia:

::: code-group

```ts{8} [src/index.ts]
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia';

const app = createApp(App);

const pinia = createPinia();
pinia.use(createPlugin());

app.use(pinia).mount('#app');
```

:::

::: tip
`createPlugin` is also exported as `TauriPluginPinia`.
:::

4. Define your Pinia store:

::: code-group

```ts [src/stores/counter.ts]
import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useCounterStore = defineStore('counter', () => {
  const counter = ref(0);

  function increment() {
    counter.value++;
  }

  return {
    counter,
    increment,
  };
});
```

:::

5. Start the plugin:

::: code-group

```ts{4} [src/App.vue]
import { useCounterStore } from './stores/counter';

const counterStore = useCounterStore();
counterStore.$tauri.start();
```

:::

::: tip
The stores won't be saved nor synchronized until you call the `start` method.
:::

## Nuxt

If you are using [Nuxt](https://nuxt.com/), you can create a plugin to enable it for Pinia:

::: code-group

```ts{6} [plugins/pinia.ts]
import type { Pinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia';

// See: https://pinia.vuejs.org/core-concepts/plugins.html#Nuxt-js
export default defineNuxtPlugin(({ $pinia }) => {
  ($pinia as Pinia).use(TauriPluginPinia());
});
```

:::
