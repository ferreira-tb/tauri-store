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
    const counter = ref(0);

    function increment() {
      counter.value++;
    }

    return {
      counter,
      increment
    };
  },
  { tauri: { debounce: 1000 } }
);
