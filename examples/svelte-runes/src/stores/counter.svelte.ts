import { open } from '@tauri-apps/plugin-shell';
import { RuneStore } from 'tauri-plugin-svelte/src/lib/index.js';
import { counter, counterOptions } from 'example-shared-js/src/index.js';

export const store = new RuneStore('counter-store', counter, counterOptions);

export async function openStore() {
  const path = await store.getPath();
  await open(path);
}
