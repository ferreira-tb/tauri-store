import { open } from '@tauri-apps/plugin-shell';
import {
  type TauriPluginValtioStoreOptions,
  store as valtio,
} from 'tauri-plugin-valtio/src/index.js';

const initialValue = {
  debouncedCounter: 0,
  debouncedCounter2: 0,
  debouncedCounter3: 0,
};

const options: TauriPluginValtioStoreOptions = {
  filterKeys: ['debouncedCounter3'],
  filterKeysStrategy: 'omit',
  saveOnChange: true,
  syncStrategy: 'debounce',
  syncInterval: 1000,
};

export const debouncedStore = valtio('debounced-counter-store', initialValue, options);

export async function openDebouncedStore() {
  const path = await debouncedStore.getPath();
  await open(path);
}

export const incrementDebouncedCounter = () => {
  debouncedStore.state.debouncedCounter += 1;
};
