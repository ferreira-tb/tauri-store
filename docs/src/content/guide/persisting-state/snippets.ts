import { pascalCase, snakeCase } from 'change-case';
import { snippet, snippetGroup } from '$stores/snippet';

export const saveStores = snippetGroup((metadata) => {
  const title = snakeCase(metadata.title!);
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
manager.${title}().save("my-store");

// Save some stores.
manager.${title}().save_some(&["my-store", "my-store-2"]);

// Save all stores.
manager.${title}().save_all();
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

    case 'tauri-plugin-valtio': {
      return `
import { store } from 'tauri-plugin-valtio';

const value = { counter: 0 };
const store = store('store', value, {
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

export const autosave = snippetGroup((metadata) => {
  return {
    id: 'autosave',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
use std::time::Duration;

// Save every five minutes.
${snakeCase(metadata.name)}::Builder::new()
  .autosave(Duration::from_secs(300))
  .build();
  `,
  };
});

export const customDirectory = snippetGroup((metadata) => {
  return {
    id: 'custom-directory',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
${snakeCase(metadata.name)}::Builder::new()
  .path("/path/to/custom/directory")
  .build();
  `,
  };
});

export const setCollectionPath = snippetGroup((metadata) => {
  const title = snakeCase(metadata.title!);
  const pascalTitle = pascalCase(metadata.title ?? '');
  return [
    {
      id: 'set-collection-path-ts',
      label: 'JavaScript',
      lang: 'typescript',
      value: `
import { set${pascalTitle}Path } from '${metadata.name}';

await set${pascalTitle}Path('/path/to/new/directory');
      `,
    },
    {
      id: 'set-collection-path-rs',
      label: 'Rust',
      lang: 'rust',
      value: `
use ${snakeCase(metadata.name)}::ManagerExt;

manager.${title}().set_path("/path/to/new/directory");
`,
    },
  ];
});

export const saveDenylist = snippetGroup((metadata) => {
  return {
    id: 'save-denylist',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
${snakeCase(metadata.name)}::Builder::new()
  .save_denylist(&["store-1", "store-2"])
  .build();
  `,
  };
});
