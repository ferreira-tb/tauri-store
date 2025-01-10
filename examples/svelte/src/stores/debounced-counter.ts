import { open } from '@tauri-apps/plugin-shell';
import { Store, type TauriPluginSvelteStoreOptions } from 'tauri-plugin-svelte/src/index.js';

const initialValue = {
  debouncedCounter: 0,
  debouncedCounter2: 0,
  debouncedCounter3: 0,
};

const options: TauriPluginSvelteStoreOptions = {
  filterKeys: ['debouncedCounter3'],
  filterKeysStrategy: 'omit',
  saveOnChange: true,
  syncStrategy: 'debounce',
  syncInterval: 1000,
};

export const debouncedStore = new Store('debounced-counter-store', initialValue, options);

export async function openDebouncedStore() {
  const path = await debouncedStore.getPath();
  await open(path);
}
