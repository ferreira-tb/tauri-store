import { open } from '@tauri-apps/plugin-shell';
import { Store } from 'tauri-store/src/index.js';

const counter = {
  counter: 0,
};

export const store = new Store('counter-store', counter, {
  autoStart: true,
  saveStrategy: 'debounce',
  saveInterval: 1000,
  hooks: {
    beforeBackendSync: (state) => {
      console.log(state);
      return state;
    },
  },
});

export async function openStore() {
  const path = await store.getPath();
  await open(path);
}

export function increment() {
  store.update('counter', (it) => ++it);
}
