import { open } from '@tauri-apps/plugin-shell';
import { RuneStore } from 'tauri-plugin-svelte/src/lib/index.js';
import { debouncedCounter, debouncedCounterOptions } from 'example-shared-js/src/index.js';

export const debouncedStore = new RuneStore(
  'debounced-counter-store',
  debouncedCounter,
  debouncedCounterOptions
);

export async function openDebouncedStore() {
  const path = await debouncedStore.getPath();
  await open(path);
}
