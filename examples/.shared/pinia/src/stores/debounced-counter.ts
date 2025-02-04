import { ref } from 'vue';
import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-shell';
import { debouncedCounterOptions, debouncedCounter as value } from 'example-shared-js/src/index.js';

function debouncedStore() {
  const debouncedCounter = ref(value.debouncedCounter);
  const debouncedCounter2 = ref(value.debouncedCounter2);
  const debouncedCounter3 = ref(value.debouncedCounter3);

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
  tauri: debouncedCounterOptions,
});

export async function openDebouncedStore() {
  const path = await useDebouncedStore().$tauri.getPath();
  await open(path);
}
