---
layout: doc
title: Getting started
titleTemplate: '@tauri-store/vue'
description: Getting started
---

# Getting started

::: info Supported Tauri Version
This plugin requires Tauri 2.0 or later.
:::

## Install

Install the [Rust crate](https://crates.io/crates/tauri-plugin-vue) by adding the following to your `Cargo.toml` file:

::: code-group

```toml [src-tauri/Cargo.toml]
[dependencies]
tauri-plugin-vue = "2"
```

:::

Install the [JavaScript package](https://www.npmjs.com/package/@tauri-store/vue) with your preferred package manager:

::: code-group

```shell [npm]
npm install @tauri-store/vue
```

```shell [pnpm]
pnpm add @tauri-store/vue
```

```shell [deno]
deno add npm:@tauri-store/vue
```

```shell [bun]
bun add @tauri-store/vue
```

```shell [yarn]
yarn add @tauri-store/vue
```

:::

## Usage

1. Enable the required permissions in your [capabilities](https://tauri.app/security/capabilities/) file:

::: code-group

```json [src-tauri/capabilities/vue.json]
{
  "identifier": "vue",
  "windows": ["*"],
  "permissions": ["vue:default", "core:event:default"]
}
```

:::

2. Register the plugin with Tauri:

::: code-group

```rust{2} [src-tauri/src/lib.rs]
tauri::Builder::default()
  .plugin(tauri_plugin_vue::init())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

:::

3. Create a store:

::: code-group

```typescript [src/lib/stores/counter.ts]
import { createStore } from '@tauri-store/vue';

export const store = createStore('counter', { counter: 0 });
```

:::

::: tip
[`createStore`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/functions/createStore.html) is also exported as [`store`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/variables/store.html).
:::

4. Start it:

```typescript
await store.$tauri.start();
```

::: tip
Stores won't be saved nor synchronized until you [start](https://tb.dev.br/tauri-store/js-docs/plugin-vue/classes/Store.html#start) them, but you can enable the [`autoStart`](https://tb.dev.br/tauri-store/js-docs/plugin-vue/interfaces/StoreFrontendOptions.html#autostart) option to make them start automatically.
:::

5. Use the store in your Vue components:

```vue
<script>
import { store } from '@/stores/counter';
</script>

<template>
  <div>
    <p>Counter: {{ store.counter }}</p>
    <button type="button" @click="() => store.counter++">
      <span>Increment</span>
    </button>
  </div>
</template>
```
