import { createStore } from '@tauri-store/vue/src/index.js';

export const vueStore = createStore(
  'playground',
  { counter: 0 },
  {
    saveOnExit: true,
    saveOnChange: true,
  }
);

Object.defineProperty(window, '$vueStore', {
  value: vueStore,
  enumerable: true,
  configurable: false,
  writable: false,
});
