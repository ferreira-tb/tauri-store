# Synchronization

Whenever the state changes, the store sends a notification to Rust to keep them in sync. Since data is serialized with each notification, frequent updates can be resource-intensive. To optimize this process, we can apply debouncing or throttling to manage synchronization more efficiently.

::: code-group

```ts [Debounce]
function debounced() {
  const counter = ref(0);

  return {
    counter,
  };
}

export const useDebouncedStore = defineStore('store', debounced, {
  tauri: {
    syncStrategy: 'debounce',
    syncInterval: 1000,
  },
});
```

```ts [Throttle]
function throttled() {
  const counter = ref(0);

  return {
    counter,
  };
}

export const useThrottledStore = defineStore('store', throttled, {
  tauri: {
    syncStrategy: 'throttle',
    syncInterval: 1000,
  },
});
```

:::

::: tip
For a detailed explanation of the differences between debouncing and throttling, take a look at [this article](https://kettanaito.com/blog/debounce-vs-throttle).
:::

It’s important to clarify that, although this process isn’t directly related to the [store's persistence](./persisting-state.md), it can still affect what gets persisted. When a store is saved, the data written to disk reflects the current state of Rust’s cache. If synchronization is still in progress, Rust might still be working with outdated values.
