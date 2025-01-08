import { open } from '@tauri-apps/plugin-shell';
import { Store, type TauriPluginSvelteStoreOptions } from 'tauri-plugin-svelte/src/index.js';

const initialValue = {
  counter: 0,
  counter2: 0,
  counter3: 0,
  nested: { foo: { bar: { baz: 0 } } },
};

const options: TauriPluginSvelteStoreOptions = {
  filterKeys: ['counter', 'counter2', 'nested'],
  filterKeysStrategy: 'pick',
  saveOnChange: true,
  saveStrategy: 'debounce',
  saveInterval: 1000,
};

export const store = new Store('counter-store', initialValue, options);

export async function openStore() {
  const path = await store.getPath();
  await open(path);
}
