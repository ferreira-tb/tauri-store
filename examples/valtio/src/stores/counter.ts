import { open } from '@tauri-apps/plugin-shell';
import { store as valtio } from 'tauri-plugin-valtio/src/index.js';
import { counter, counterOptions } from 'example-shared-js/src/index.js';

export const store = valtio('counter-store', counter, counterOptions);

export async function openStore() {
  const path = await store.getPath();
  await open(path);
}

export const incrementCounter = () => {
  store.state.counter += 1;
};
