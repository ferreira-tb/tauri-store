---
layout: doc
title: Getting started
titleTemplate: '@tauri-store/svelte'
description: Getting started
---

# Getting started

::: info Supported Tauri Version
This plugin requires Tauri 2.0 or later.
:::

## Install

Install the [Rust crate](https://crates.io/crates/tauri-plugin-svelte) by adding the following to your `Cargo.toml` file:

::: code-group

```toml [src-tauri/Cargo.toml]
[dependencies]
tauri-plugin-svelte = "2"
```

:::

Install the [JavaScript package](https://www.npmjs.com/package/@tauri-store/svelte) with your preferred package manager:

::: code-group

```shell [npm]
npm install @tauri-store/svelte
```

```shell [pnpm]
pnpm add @tauri-store/svelte
```

```shell [deno]
deno add npm:@tauri-store/svelte
```

```shell [bun]
bun add @tauri-store/svelte
```

```shell [yarn]
yarn add @tauri-store/svelte
```

:::

## Usage

1. Enable the required permissions in your [capabilities](https://tauri.app/security/capabilities/) file:

::: code-group

```json [src-tauri/capabilities/svelte.json]
{
  "identifier": "svelte",
  "windows": ["*"],
  "permissions": ["svelte:default", "core:event:default"]
}
```

:::

2. Register the plugin with Tauri:

::: code-group

```rust [src-tauri/src/lib.rs]
tauri::Builder::default()
  .plugin(tauri_plugin_svelte::init())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

:::

3. Create a store:

::: code-group

```typescript [src/lib/stores/counter.ts]
import { Store } from '@tauri-store/svelte';

export const store = new Store('counter', { counter: 0 });
```

:::

4. Start it:

```typescript
await store.start();
```

::: tip
Stores won't be saved nor synchronized until you call the [start](https://tb.dev.br/tauri-store/js-docs/plugin-svelte/classes/Store.html#start) method.
:::

5. Use the store in your Svelte components:

```svelte
<script>
  import { store } from '$lib/stores/counter';
</script>

<div>
  <p>Counter: {$store.counter}</p>
  <button type="button" onclick={() => $store.counter++}>
    <span>Increment</span>
  </button>
</div>
```

## Runes

You can also use [runes](https://svelte.dev/docs/svelte/what-are-runes) instead of conventional stores.

::: code-group

```typescript [src/lib/counter.svelte.ts]
import { RuneStore } from '@tauri-store/svelte';

const store = new RuneStore('counter', { counter: 0 });

function increment() {
  // "state" is reactive!
  store.state.counter += 1;
}
```
