import { ref } from 'vue';
import { defineStore } from 'pinia';

function store() {
  const counter = ref<number>(0);

  return {
    counter,
  };
}

export const useStore = defineStore('playground', store, {
  tauri: {
    saveOnExit: true,
    saveOnChange: true,
    saveStrategy: 'debounce',
    saveInterval: 1000,
  },
});

if (!Object.hasOwn(window, '$store')) {
  Object.defineProperty(window, '$store', {
    value: useStore,
    enumerable: true,
    configurable: false,
    writable: false,
  });
}
