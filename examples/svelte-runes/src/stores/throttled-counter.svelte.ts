import { open } from '@tauri-apps/plugin-shell';
import { RuneStore } from 'tauri-plugin-svelte/src/lib/index.js';
import { throttledCounter, throttledCounterOptions } from 'example-shared-js/src/index.js';

export const throttledStore = new RuneStore(
  'throttled-counter-store',
  throttledCounter,
  throttledCounterOptions
);

export async function openThrottledStore() {
  const path = await throttledStore.getPath();
  await open(path);
}
