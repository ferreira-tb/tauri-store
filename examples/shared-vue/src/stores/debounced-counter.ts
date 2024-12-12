import { ref } from 'vue';
import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-shell';

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
    saveOnChange: true,
    syncStrategy: 'debounce',
    syncInterval: 1000,
  },
});

export async function openDebouncedStore() {
  const path = await useDebouncedStore().$tauri.getPath();
  await open(path);
}
