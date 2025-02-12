import { pascalCase, snakeCase } from 'change-case';
import { snippet, snippetGroup } from '$stores/snippet';

export const saveStores = snippetGroup((metadata, ctx) => {
  return [
    {
      id: 'save-stores-ts',
      label: 'JavaScript',
      lang: 'typescript',
      value: `
import { save, saveAll } from '${metadata.name}';

// Save a single store.
await save('my-store');

// Save some stores.
await save('store-1', 'store-2', 'store-3');

// Save all stores.
await saveAll();
      `,
    },
    {
      id: 'save-stores-rs',
      label: 'Rust',
      lang: 'rust',
      value: `
use ${snakeCase(metadata.name)}::ManagerExt;

// Here, "manager" represents any type that implements the "Manager" trait provided by Tauri.
// This includes "AppHandle", "Window", and "WebviewWindow".
// See: https://docs.rs/tauri/latest/tauri/trait.Manager.html

// Save a single store.
manager.${ctx.collection}().save("my-store");

// Save some stores.
manager.${ctx.collection}().save_some(&["my-store", "my-store-2"]);

// Save all stores.
manager.${ctx.collection}().save_all();
      `,
    },
  ];
});

export const saveOnChange = snippet((metadata) => {
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
    saveOnChange: true,

    // You can also debounce or throttle when saving.
    // This is optional. The default behavior is to save immediately.
    saveStrategy: 'debounce',
    saveInterval: 1000,
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

export const autosave = snippetGroup((metadata, ctx) => {
  return {
    id: 'autosave',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
use std::time::Duration;

// Save every five minutes.
${snakeCase(metadata.name)}::Builder::new()
  .autosave(Duration::from_secs(300))
  .${ctx.isTauriStore ? 'build_plugin' : 'build'}();
  `,
  };
});

export const customDirectory = snippetGroup((metadata, ctx) => {
  return {
    id: 'custom-directory',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
${snakeCase(metadata.name)}::Builder::new()
  .path("/path/to/custom/directory")
  .${ctx.isTauriStore ? 'build_plugin' : 'build'}();
    `,
  };
});

export const setCollectionPath = snippetGroup((metadata, ctx) => {
  const collection = pascalCase(ctx.collection);
  return [
    {
      id: 'set-collection-path-ts',
      label: 'JavaScript',
      lang: 'typescript',
      value: `
import { set${collection}Path } from '${metadata.name}';

await set${collection}Path('/path/to/new/directory');
      `,
    },
    {
      id: 'set-collection-path-rs',
      label: 'Rust',
      lang: 'rust',
      value: `
use ${snakeCase(metadata.name)}::ManagerExt;

manager.${ctx.collection}().set_path("/path/to/new/directory");
`,
    },
  ];
});

export const saveDenylist = snippetGroup((metadata, ctx) => {
  return {
    id: 'save-denylist',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
${snakeCase(metadata.name)}::Builder::new()
  .save_denylist(&["store-1", "store-2"])
  .${ctx.isTauriStore ? 'build_plugin' : 'build'}();
  `,
  };
});
