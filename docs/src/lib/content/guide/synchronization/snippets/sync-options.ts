import { snippet } from '$lib/stores/snippet';

export const syncOptions = snippet((metadata) => {
  const name = metadata.name as TauriPlugin;

  switch (name) {
    case 'tauri-plugin-pinia': {
      return `
import { defineStore } from 'pinia';

function store() {
  const counter = ref(0);
  return { counter };
}

export const useStore = defineStore('store', store, {
  tauri: {
    syncStrategy: 'debounce',
    syncInterval: 1000,
  },
});
      `;
    }
    case 'tauri-plugin-svelte': {
      return `
import { Store } from 'tauri-plugin-svelte';

const value = { counter: 0 };
const store = new Store('store', value, {
  syncStrategy: 'debounce',
  syncInterval: 1000,
});
      `;
    }
  }
});
