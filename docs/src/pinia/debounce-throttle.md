# Debounce and throttle

The syncronization of a store with the Rust backend can sometimes be expensive. To help with this, we can debounce or throttle the updates.

It's important to note that this is not directly related to the [store's persistence](./persisting-state.md). This only cares about how often the store is synchronized with the backend.

::: code-group

```ts [Debounce]
function debounced() {
  const counter = ref(0);

  function increment() {
    counter.value++;
  }

  return {
    counter,
    increment,
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

  function increment() {
    counter.value++;
  }

  return {
    counter,
    increment,
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
For a detailed explanation of the difference between debounce and throttle, check [this article](https://kettanaito.com/blog/debounce-vs-throttle).
:::
