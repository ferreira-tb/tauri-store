import { open } from '@tauri-apps/plugin-shell';
import {
  type TauriPluginValtioStoreOptions,
  store as valtio,
} from 'tauri-plugin-valtio/src/index.js';

const initialValue = {
  throttledCounter: 0,
  throttledCounter2: 0,
  throttledCounter3: 0,
};

const options: TauriPluginValtioStoreOptions = {
  filterKeys: ['throttledCounter3'],
  filterKeysStrategy: 'omit',
  saveOnChange: true,
  syncStrategy: 'throttle',
  syncInterval: 1000,
};

export const throttledStore = valtio('throttled-counter-store', initialValue, options);

export async function openThrottledStore() {
  const path = await throttledStore.getPath();
  await open(path);
}

export const incrementThrottledCounter = () => {
  throttledStore.state.throttledCounter += 1;
};