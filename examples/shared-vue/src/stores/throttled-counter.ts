import { ref } from 'vue';
import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-shell';

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

export const useThrottledStore = defineStore('throttled-counter-store', throttledStore, {
  tauri: {
    filterKeys: ['throttledCounter3'],
    filterKeysStrategy: 'omit',
    syncStrategy: 'throttle',
    syncInterval: 1000,
  },
});

export async function openThrottledStore() {
  const path = await useThrottledStore().$tauri.getPath();
  await open(path);
}
