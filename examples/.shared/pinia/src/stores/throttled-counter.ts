import { ref } from 'vue';
import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-shell';
import { throttledCounterOptions, throttledCounter as value } from 'example-shared-js/src/index.js';

function throttledStore() {
  const throttledCounter = ref(value.throttledCounter);
  const throttledCounter2 = ref(value.throttledCounter2);
  const throttledCounter3 = ref(value.throttledCounter3);

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
  tauri: throttledCounterOptions,
});

export async function openThrottledStore() {
  const path = await useThrottledStore().$tauri.getPath();
  await open(path);
}
