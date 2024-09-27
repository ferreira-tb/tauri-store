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
    debounce: 1000,
    filterKeys: ['debouncedCounter3'],
    filterKeysStrategy: 'omit',
  },
});
