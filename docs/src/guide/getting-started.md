---
layout: doc
title: Getting started
description: Getting started
---

# Getting started

::: info Supported Tauri Version
This plugin requires Tauri 2.0 or later.
:::

## Install

Install the [Rust crate](https://crates.io/crates/tauri-store) by adding the following to your `Cargo.toml` file:

::: code-group

```toml [src-tauri/Cargo.toml]
[dependencies]
tauri-store = "0.10"
```

:::

Install the [JavaScript package](https://www.npmjs.com/package/tauri-store) with your preferred package manager:

::: code-group

```shell [npm]
npm install tauri-store
```

```shell [pnpm]
pnpm add tauri-store
```

```shell [deno]
deno add npm:tauri-store
```

```shell [bun]
bun add tauri-store
```

```shell [yarn]
yarn add tauri-store
```

:::

## Usage

1. Enable the required permissions in your [capabilities](https://tauri.app/security/capabilities/) file:

::: code-group

```json [src-tauri/capabilities/tauri-store.json]
{
  "identifier": "tauri-store",
  "windows": ["*"],
  "permissions": ["tauri-store:default", "core:event:default"]
}
```

:::

2. Register the plugin with Tauri:

::: code-group

```rust{2} [src-tauri/src/lib.rs]
tauri::Builder::default()
  .plugin(tauri_store::init())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

:::

3. Create a store:

::: code-group

```typescript [src/stores/counter.ts]
import { Store } from 'tauri-store';

const store = new Store('counter', { counter: 0 });
```

:::

4. Start it:

```typescript
await store.start();
```

::: tip
Stores won't be saved nor synchronized until you call the [start](https://tb.dev.br/tauri-store/js-docs/tauri-store/classes/Store.html#start) method.
:::

5. Use the store:

```typescript
// Get a value.
// This is a synchronous operation!
const counter = store.get('counter');
console.log(counter);

// Set a value.
store.set('counter', 42);

// Update a value with a callback.
store.update('counter', (value) => value + 1);

// Set multiple values at once.
store.patch({ counter: 0 });

// Listen to changes.
store.subscribe((state) => {
  console.log(state);
});

// Save the store.
// Unlike the others, this is asynchronous.
await store.save();
```
