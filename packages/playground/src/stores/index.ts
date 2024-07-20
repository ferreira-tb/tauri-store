import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useStore = defineStore('store', () => {
  const counter = ref(0);

  function increment() {
    counter.value++;
  }

  return {
    counter,
    increment
  };
});

export const useDebouncedStore = defineStore(
  'debounced-store',
  () => {
    const debouncedCounter = ref(0);

    function increment() {
      debouncedCounter.value++;
    }

    return {
      debouncedCounter,
      increment
    };
  },
  { tauri: { debounce: 1000 } }
);
