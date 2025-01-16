import { open } from '@tauri-apps/plugin-shell';
import { Store } from 'tauri-plugin-svelte/src/index.js';
import { throttledCounter, throttledCounterOptions } from 'example-shared-js/src/index.js';

export const throttledStore = new Store(
  'throttled-counter-store',
  throttledCounter,
  throttledCounterOptions
);

export async function openThrottledStore() {
  const path = await throttledStore.getPath();
  await open(path);
}
