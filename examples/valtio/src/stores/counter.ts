import { open } from '@tauri-apps/plugin-shell';
import {
  type TauriPluginValtioStoreOptions,
  store as valtio,
} from 'tauri-plugin-valtio/src/index.js';

const initialValue = {
  counter: 0,
  counter2: 0,
  counter3: 0,
  nested: { foo: { bar: { baz: 0 } } },
};

const options: TauriPluginValtioStoreOptions = {
  filterKeys: ['counter', 'counter2', 'nested'],
  filterKeysStrategy: 'pick',
  saveOnChange: true,
  saveStrategy: 'debounce',
  saveInterval: 1000,
};

export const store = valtio('counter-store', initialValue, options);

export async function openStore() {
  const path = await store.getPath();
  await open(path);
}

export const incrementCounter = () => {
  store.state.counter += 1;
};
