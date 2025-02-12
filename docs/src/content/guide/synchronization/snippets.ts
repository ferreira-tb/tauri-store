import { snakeCase } from 'change-case';
import { snippet, snippetGroup } from '$stores/snippet';

export const syncDenylist = snippetGroup((metadata, ctx) => {
  return {
    id: 'sync-denylist',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
${snakeCase(metadata.name)}::Builder::new()
  .sync_denylist(&["store-1", "store-2"])
  .${ctx.isTauriStore ? 'build_plugin' : 'build'}()
  `,
  };
});

export const syncOptions = snippet((metadata) => {
  const name = metadata.name as TauriPlugin;
  switch (name) {
    case 'tauri-plugin-pinia': {
      return `
import { ref } from 'vue';
import { defineStore } from 'pinia';

function counterStore() {
  return { counter: ref(0) };
}

export const useCounterStore = defineStore('counter', counterStore, {
  tauri: {
    syncStrategy: 'debounce',
    syncInterval: 1000,
  },
});
      `;
    }

    case 'tauri-plugin-svelte':
    case 'tauri-plugin-valtio':
    case 'tauri-store': {
      return `
import { store } from '${name}';

const value = { counter: 0 };
const counterStore = store('counter', value, {
  syncStrategy: 'debounce',
  syncInterval: 1000,
});
      `;
    }
  }
});
