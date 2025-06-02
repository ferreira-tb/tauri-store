import { ref } from 'vue';
import { defineStore } from 'pinia';

function store() {
  const counter = ref<number>(0);

  return {
    counter,
  };
}

export const usePiniaStore = defineStore('playground', store, {
  tauri: {
    autoStart: true,
    saveOnExit: true,
    saveOnChange: true,
  },
});

Object.defineProperty(window, '$piniaStore', {
  value: usePiniaStore,
  enumerable: true,
  configurable: false,
  writable: false,
});
