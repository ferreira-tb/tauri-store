import { snakeCase } from 'change-case';
import { snippet, snippetGroup } from '$stores/snippet';

export const syncDenylist = snippetGroup((metadata) => {
  return {
    id: 'sync-denylist',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
${snakeCase(metadata.name)}::Builder::new()
  .sync_denylist(&["store-1", "store-2"])
  .build();
  `,
  };
});

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

    case 'tauri-plugin-valtio': {
      return `
import { store } from 'tauri-plugin-valtio';

const value = { counter: 0 };
const store = store('store', value, {
  syncStrategy: 'debounce',
  syncInterval: 1000,
});
      `;
    }
  }
});
