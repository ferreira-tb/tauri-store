import { open } from '@tauri-apps/plugin-shell';
import { store as valtio } from 'tauri-plugin-valtio/src/index.js';
import { debouncedCounter, debouncedCounterOptions } from 'example-shared-js/src/index.js';

export const debouncedStore = valtio(
  'debounced-counter-store',
  debouncedCounter,
  debouncedCounterOptions
);

export async function openDebouncedStore() {
  const path = await debouncedStore.getPath();
  await open(path);
}

export const incrementDebouncedCounter = () => {
  debouncedStore.state.debouncedCounter += 1;
};
