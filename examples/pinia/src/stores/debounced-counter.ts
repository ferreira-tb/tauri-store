import { ref } from 'vue';
import { defineStore } from 'pinia';

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

export const useDebouncedStore = defineStore('debounced-counter-store', debouncedStore, {
  tauri: {
    filterKeys: ['debouncedCounter3'],
    filterKeysStrategy: 'omit',
    syncStrategy: 'debounce',
    syncInterval: 1000,
  },
});
