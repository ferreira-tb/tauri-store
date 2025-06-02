---
layout: doc
title: Getting started
titleTemplate: '@tauri-store/pinia'
description: Getting started
---

# Getting started

::: info Supported Tauri Version
This plugin requires Tauri 2.0 or later.
:::

## Install

Install the [Rust crate](https://crates.io/crates/tauri-plugin-pinia) by adding the following to your `Cargo.toml` file:

::: code-group

```toml [src-tauri/Cargo.toml]
[dependencies]
tauri-plugin-pinia = "3"
```

:::

Install the [JavaScript package](https://www.npmjs.com/package/@tauri-store/pinia) with your preferred package manager:

::: code-group

```shell [npm]
npm install @tauri-store/pinia
```

```shell [pnpm]
pnpm add @tauri-store/pinia
```

```shell [deno]
deno add npm:@tauri-store/pinia
```

```shell [bun]
bun add @tauri-store/pinia
```

```shell [yarn]
yarn add @tauri-store/pinia
```

:::

## Usage

1. Enable the required permissions in your [capabilities](https://tauri.app/security/capabilities/) file:

::: code-group

```json [src-tauri/capabilities/pinia.json]
{
  "identifier": "pinia",
  "windows": ["*"],
  "permissions": ["pinia:default", "core:event:default"]
}
```

:::

2. Register the plugin with Tauri:

::: code-group

```rust{2} [src-tauri/src/lib.rs]
tauri::Builder::default()
  .plugin(tauri_plugin_pinia::init())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

:::

3. Enable the plugin for Pinia:

::: code-group

```typescript [src/index.ts]
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createPlugin } from '@tauri-store/pinia';

const app = createApp(App);

const pinia = createPinia();
pinia.use(createPlugin());

app.use(pinia);
app.mount('#app');
```

::: tip
[`createPlugin`](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/functions/createPlugin.html) is also exported as [`TauriPluginPinia`](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/variables/TauriPluginPinia.html).
:::

4. Define your Pinia store:

::: code-group

```typescript [src/stores/counter.ts]
import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useStore = defineStore('counter', () => {
  const counter = ref(0);
  return { counter };
});
```

:::

5. Start it:

```typescript
const store = useStore();
await store.$tauri.start();
```

::: tip
Stores won't be saved nor synchronized until you [start](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/interfaces/TauriStoreContract.html#start) them, but you can enable the [`autoStart`](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/interfaces/StoreFrontendOptions.html#autostart) option to make them start automatically.
:::

## Nuxt

If you are using [Nuxt](https://nuxt.com/), you can create a [Nuxt plugin](https://nuxt.com/docs/guide/directory-structure/plugins) to enable it for Pinia:

::: code-group

```typescript [plugins/pinia.ts]
import type { Pinia } from 'pinia';
import { TauriPluginPinia } from '@tauri-store/pinia';

// See: https://pinia.vuejs.org/core-concepts/plugins.html#Nuxt-js
export default defineNuxtPlugin(({ $pinia }) => {
  ($pinia as Pinia).use(TauriPluginPinia());
});
```

:::
