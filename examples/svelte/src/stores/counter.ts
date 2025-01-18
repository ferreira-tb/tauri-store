import { open } from '@tauri-apps/plugin-shell';
import { Store } from 'tauri-plugin-svelte/src/lib/index.js';
import { counter, counterOptions } from 'example-shared-js/src/index.js';

export const store = new Store('counter-store', counter, counterOptions);

export async function openStore() {
  const path = await store.getPath();
  await open(path);
}
