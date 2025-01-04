import { snippet } from '$lib/stores/snippet';

export const saveOnChange = snippet((metadata) => {
  const name = metadata.name as TauriPlugin;
  switch (name) {
    case 'tauri-plugin-pinia': {
      return `
import { ref } from 'vue';
import { defineStore } from 'pinia';

function store() {
  const counter = ref(0);
  return { counter };
}

export const useStore = defineStore('store', store, {
  tauri: {
    saveOnChange: true,

    // You can also debounce or throttle when saving.
    // This is optional. The default behavior is to save immediately.
    saveStrategy: 'debounce',
    saveInterval: 1000,
  },
});
      `;
    }

    case 'tauri-plugin-svelte': {
      return `
import { Store } from 'tauri-plugin-svelte';

const value = { counter: 0 };
const store = new Store('store', value, {
  saveOnChange: true,

  // You can also debounce or throttle when saving.
  // This is optional. The default behavior is to save immediately.
  saveStrategy: 'debounce',
  saveInterval: 1000,
});
      `;
    }
  }
});
