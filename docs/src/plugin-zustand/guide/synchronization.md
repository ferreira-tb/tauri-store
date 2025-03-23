---
layout: doc
title: Synchronization
titleTemplate: '@tauri-store/zustand'
description: Synchronization
---

# Synchronization

Whenever the state changes, the store notifies Rust to keep the frontend and backend in sync. However, since data is serialized with each notification, frequent updates can be resource-intensive. One way to address this issue is by applying debouncing or throttling, making the synchronization process more efficient.

```typescript
import { create } from 'zustand';
import { createTauriStore } from '@tauri-store/zustand';

const useCounterStore = create<{ counter: number }>((set) => ({
  counter: 0,
}));

const tauriHandler = createTauriStore('counter', useCounterStore, {
  syncStrategy: 'debounce',
  syncInterval: 1000,
});
```

::: tip Debounce or throttle?
For a detailed explanation of the differences between debouncing and throttling, take a look at [this article](https://kettanaito.com/blog/debounce-vs-throttle).
:::

While this process isn’t directly related to [store persistence](./persisting-state.md), it can still affect what gets saved. When a store is saved, the data written to disk comes from Rust’s cache at that moment. If the synchronization hasn’t finished yet, Rust might still be working with outdated values.

## Denylist

If a store should be [saved to disk](./persisting-state.md), but not synchronized across windows, you can add it to the [denylist](https://docs.rs/tauri-plugin-zustand/latest/tauri_plugin_zustand/struct.Builder.html#method.sync_denylist).

::: code-group

```rust [src-tauri/src/lib.rs]
tauri_plugin_zustand::Builder::new()
  .sync_denylist(&["store-1", "store-2"])
  .build()
```

:::
