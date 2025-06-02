---
layout: doc
title: Synchronization
titleTemplate: '@tauri-store/pinia'
description: Synchronization
---

# Synchronization

Whenever the state changes, the store notifies Rust to keep the frontend and backend in sync. However, since data is serialized with each notification, frequent updates can be resource-intensive. One way to address this issue is by applying debouncing or throttling, making the synchronization process more efficient.

```typescript{10-11}
import { ref } from 'vue';
import { defineStore } from 'pinia';

function counterStore() {
  return { counter: ref(0) };
}

export const useCounterStore = defineStore('counter', counterStore, {
  tauri: {
    syncStrategy: 'debounce',
    syncInterval: 1000,
  },
});
```

::: tip Debounce or throttle?
For a detailed explanation of the differences between debouncing and throttling, take a look at [this article](https://kettanaito.com/blog/debounce-vs-throttle).
:::

While this process isn’t directly related to [store persistence](./persisting-state.md), it can still affect what gets saved. When a store is saved, the data written to disk comes from Rust’s cache at that moment. If the synchronization hasn’t finished yet, Rust might still be working with outdated values.

## Filtering keys

For finer control over which keys are synced with the backend, you can set a filter strategy when defining the store.

```typescript{13-14}
import { ref } from 'vue';
import { defineStore } from 'pinia';

function counterStore() {
  return {
    counter: ref(0),
    ignoreMe: ref('hello darkness, my old friend')
  };
}

export const useCounterStore = defineStore('counter', counterStore, {
  tauri: {
    filterKeys: ['ignoreMe'],
    filterKeysStrategy: 'omit',
  },
});
```

::: tip
[`filterKeysStrategy`](https://tb.dev.br/tauri-store/js-docs/plugin-pinia/interfaces/StoreFrontendOptions.html#filterkeysstrategy) can also accept a callback to dynamically check if the key should be filtered.
:::

## Denylist

If a store should be [saved to disk](./persisting-state.md), but not synchronized across windows, you can add it to the [denylist](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.Builder.html#method.sync_denylist).

::: code-group

```rust{2} [src-tauri/src/lib.rs]
tauri_plugin_pinia::Builder::new()
  .sync_denylist(&["store-1", "store-2"])
  .build()
```

:::
