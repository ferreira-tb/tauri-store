---
layout: doc
title: Synchronization
titleTemplate: '@tauri-store/valtio'
description: Synchronization
---

# Synchronization

Whenever the state changes, the store notifies Rust to keep the frontend and backend in sync. However, since data is serialized with each notification, frequent updates can be resource-intensive. One way to address this issue is by applying debouncing or throttling, making the synchronization process more efficient.

```typescript{5-6}
import { store } from '@tauri-store/valtio';

const value = { counter: 0 };
const counterStore = store('counter', value, {
  syncStrategy: 'debounce',
  syncInterval: 1000,
});
```

::: tip Debounce or throttle?
For a detailed explanation of the differences between debouncing and throttling, take a look at [this article](https://kettanaito.com/blog/debounce-vs-throttle).
:::

While this process isn’t directly related to [store persistence](./persisting-state.md), it can still affect what gets saved. When a store is saved, the data written to disk comes from Rust’s cache at that moment. If the synchronization hasn’t finished yet, Rust might still be working with outdated values.

## Filtering keys

For finer control over which keys are synced with the backend, you can set a filter strategy when defining the store.

```typescript{5-6}
import { store } from '@tauri-store/valtio';

const value = { counter: 0, ignoreMe: 'hello darkness, my old friend' };
const counterStore = store('counter', value, {
  filterKeys: ['ignoreMe'],
  filterKeysStrategy: 'omit',
});
```

::: tip
[`filterKeysStrategy`](https://tb.dev.br/tauri-store/js-docs/plugin-valtio/interfaces/StoreFrontendOptions.html#filterkeysstrategy) can also accept a callback to dynamically check if the key should be filtered.
:::

## Denylist

If a store should be [saved to disk](./persisting-state.md), but not synchronized across windows, you can add it to the [denylist](https://docs.rs/tauri-plugin-valtio/latest/tauri_plugin_valtio/struct.Builder.html#method.sync_denylist).

::: code-group

```typescript{3} [JavaScript]
import { denySync, allowSync } from '@tauri-store/valtio';

await denySync('store-1', 'store-2');

// To allow them again:
await allowSync('store-1', 'store-2');
```

```rust{2} [Rust]
tauri_plugin_valtio::Builder::new()
  .sync_denylist(&["store-1", "store-2"])
  .build();
```

:::
