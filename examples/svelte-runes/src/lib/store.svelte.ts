import { open } from '@tauri-apps/plugin-shell';
import { RuneStore } from '@tauri-store/svelte/src/lib/index.js';

type Counter = {
  counter: number;
};

const defaultValue: Counter = {
  counter: 0,
};

export const store = new RuneStore('counter-store', defaultValue, {
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
