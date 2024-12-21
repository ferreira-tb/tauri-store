import { open } from '@tauri-apps/plugin-shell';
import { Store, type TauriPluginSvelteStoreOptions } from 'tauri-plugin-svelte/src/index.js';

const initialValue = {
  throttledCounter: 0,
  throttledCounter2: 0,
  throttledCounter3: 0,
};

const options: TauriPluginSvelteStoreOptions = {
  filterKeys: ['throttledCounter3'],
  filterKeysStrategy: 'omit',
  saveOnChange: true,
  syncStrategy: 'throttle',
  syncInterval: 1000,
};

export const throttledStore = new Store('throttled-counter-store', initialValue, options);

export async function openThrottledStore() {
  const path = await throttledStore.getPath();
  await open(path);
}
