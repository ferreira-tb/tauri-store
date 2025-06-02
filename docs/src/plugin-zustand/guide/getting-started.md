---
layout: doc
title: Getting started
titleTemplate: '@tauri-store/zustand'
description: Getting started
---

# Getting started

::: info Supported Tauri Version
This plugin requires Tauri 2.0 or later.
:::

## Install

Install the [Rust crate](https://crates.io/crates/tauri-plugin-zustand) by adding the following to your `Cargo.toml` file:

::: code-group

```toml [src-tauri/Cargo.toml]
[dependencies]
tauri-plugin-zustand = "0.3"
```

:::

Install the [JavaScript package](https://www.npmjs.com/package/@tauri-store/zustand) with your preferred package manager:

::: code-group

```shell [npm]
npm install @tauri-store/zustand
```

```shell [pnpm]
pnpm add @tauri-store/zustand
```

```shell [deno]
deno add npm:@tauri-store/zustand
```

```shell [bun]
bun add @tauri-store/zustand
```

```shell [yarn]
yarn add @tauri-store/zustand
```

:::

## Usage

1. Enable the required permissions in your [capabilities](https://tauri.app/security/capabilities/) file:

::: code-group

```json [src-tauri/capabilities/zustand.json]
{
  "identifier": "zustand",
  "windows": ["*"],
  "permissions": ["zustand:default", "core:event:default"]
}
```

:::

2. Register the plugin with Tauri:

::: code-group

```rust{2} [src-tauri/src/lib.rs]
tauri::Builder::default()
  .plugin(tauri_plugin_zustand::init())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

:::

3. Create a store:

::: code-group

```typescript [src/stores/counter.ts]
import { create } from 'zustand';
import { createTauriStore } from '@tauri-store/zustand';

type CounterStore = {
  counter: number;
  increment: () => void;
};

// A Zustand store, like any other.
export const useCounterStore = create<CounterStore>((set) => ({
  counter: 0,
  increment: () => set((state) => ({ counter: state.counter + 1 })),
}));

// A handle to the Tauri plugin.
// We will need this to start the store.
export const tauriHandler = createTauriStore('counter', useCounterStore);
```

:::

4. Start it:

```typescript
await tauriHandler.start();
```

::: tip
Stores won't be saved nor synchronized until you [start](https://tb.dev.br/tauri-store/js-docs/plugin-zustand/classes/TauriStore.html#start) them, but you can enable the [`autoStart`](https://tb.dev.br/tauri-store/js-docs/plugin-zustand/interfaces/StoreFrontendOptions.html#autostart) option to make them start automatically.
:::

5. Use the store in your React components:

```tsx
import { useCounterStore } from '@/stores/counter';

export default function MyComponent() {
  const counter = useCounterStore((state) => state.counter);
  const increment = useCounterStore((state) => state.increment);

  return (
    <div>
      <p>Counter: {counter}</p>
      <button type="button" onClick={increment}>
        <span>Increment</span>
      </button>
    </div>
  );
}
```
