---
layout: doc
title: Synchronization
titleTemplate: '@tauri-store/svelte'
description: Synchronization
---

# Synchronization

Whenever the state changes, the store notifies Rust to keep the frontend and backend in sync. However, since data is serialized with each notification, frequent updates can be resource-intensive. One way to address this issue is by applying debouncing or throttling, making the synchronization process more efficient.

```typescript{5-6}
import { store } from '@tauri-store/svelte';

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

## Denylist

If a store should be [saved to disk](./persisting-state.md), but not synchronized across windows, you can add it to the [denylist](https://docs.rs/tauri-plugin-svelte/latest/tauri_plugin_svelte/struct.Builder.html#method.sync_denylist).

::: code-group

```rust{2} [src-tauri/src/lib.rs]
tauri_plugin_svelte::Builder::new()
  .sync_denylist(&["store-1", "store-2"])
  .build()
```

:::
