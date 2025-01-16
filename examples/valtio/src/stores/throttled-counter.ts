import { open } from '@tauri-apps/plugin-shell';
import { store as valtio } from 'tauri-plugin-valtio/src/index.js';
import { throttledCounter, throttledCounterOptions } from 'example-shared-js/src/index.js';

export const throttledStore = valtio(
  'throttled-counter-store',
  throttledCounter,
  throttledCounterOptions
);

export async function openThrottledStore() {
  const path = await throttledStore.getPath();
  await open(path);
}

export const incrementThrottledCounter = () => {
  throttledStore.state.throttledCounter += 1;
};
