import { ref } from 'vue';
import { defineStore } from 'pinia';

function store() {
  const counter = ref(0);
  const counter2 = ref(0);
  const counter3 = ref(0);

  const nested = ref({ foo: { bar: { baz: 0 } } });

  function increment() {
    counter.value++;
  }

  return {
    counter,
    counter2,
    counter3,
    nested,
    increment,
  };
}

export const useStore = defineStore('store', store, {
  tauri: {
    filterKeys: ['counter', 'counter2', 'nested'],
    filterKeysStrategy: 'pick',
  },
});

function debouncedStore() {
  const debouncedCounter = ref(0);
  const debouncedCounter2 = ref(0);
  const debouncedCounter3 = ref(0);

  function increment() {
    debouncedCounter.value++;
  }

  return {
    debouncedCounter,
    debouncedCounter2,
    debouncedCounter3,
    increment,
  };
}

export const useDebouncedStore = defineStore('debounced-store', debouncedStore, {
  tauri: {
    filterKeys: ['debouncedCounter3'],
    filterKeysStrategy: 'omit',
    syncStrategy: 'debounce',
    syncInterval: 1000,
  },
});

function throttledStore() {
  const throttledCounter = ref(0);
  const throttledCounter2 = ref(0);
  const throttledCounter3 = ref(0);

  function increment() {
    throttledCounter.value++;
  }

  return {
    throttledCounter,
    throttledCounter2,
    throttledCounter3,
    increment,
  };
}

export const useThrottledStore = defineStore('throttled-store', throttledStore, {
  tauri: {
    filterKeys: ['throttledCounter3'],
    filterKeysStrategy: 'omit',
    syncStrategy: 'throttle',
    syncInterval: 1000,
  },
});
