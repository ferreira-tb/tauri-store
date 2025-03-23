---
layout: doc
title: Getting started
titleTemplate: '@tauri-store/valtio'
description: Getting started
---

# Getting started

::: info Supported Tauri Version
This plugin requires Tauri 2.0 or later.
:::

## Install

Install the [Rust crate](https://crates.io/crates/tauri-plugin-valtio) by adding the following to your `Cargo.toml` file:

::: code-group

```toml [src-tauri/Cargo.toml]
[dependencies]
tauri-plugin-valtio = "2"
```

:::

Install the [JavaScript package](https://www.npmjs.com/package/@tauri-store/valtio) with your preferred package manager:

::: code-group

```shell [npm]
npm install @tauri-store/valtio
```

```shell [pnpm]
pnpm add @tauri-store/valtio
```

```shell [deno]
deno add npm:@tauri-store/valtio
```

```shell [bun]
bun add @tauri-store/valtio
```

```shell [yarn]
yarn add @tauri-store/valtio
```

:::

## Usage

1. Enable the required permissions in your [capabilities](https://tauri.app/security/capabilities/) file:

::: code-group

```json [src-tauri/capabilities/valtio.json]
{
  "identifier": "valtio",
  "windows": ["*"],
  "permissions": ["valtio:default", "core:event:default"]
}
```

:::

2. Register the plugin with Tauri:

::: code-group

```rust [src-tauri/src/lib.rs]
tauri::Builder::default()
  .plugin(tauri_plugin_valtio::init())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

:::

3. Create a store:

::: code-group

```typescript [src/stores/counter.ts]
import { store } from '@tauri-store/valtio';

export const counterStore = store('counter', { counter: 0 });

export const increment = () => {
  counterStore.state.counter++;
};
```

:::

4. Start it:

```typescript
await counterStore.start();
```

::: tip
Stores won't be saved nor synchronized until you call the [start](https://tb.dev.br/tauri-store/js-docs/plugin-valtio/classes/Store.html#start) method.
:::

5. Use the store in your React components:

```tsx
import { useSnapshot } from 'valtio';
import { counterStore, increment } from '@/stores/counter';

export default function MyComponent() {
  // `state` is the actual valtio proxy.
  const snap = useSnapshot(counterStore.state);

  return (
    <div>
      <p>Counter: {snap.counter}</p>
      <button type="button" onClick={increment}>
        <span>Increment</span>
      </button>
    </div>
  );
}
```
