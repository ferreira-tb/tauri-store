import { open } from '@tauri-apps/plugin-shell';
import { createStore } from '@tauri-store/vue/src/index.js';

type Counter = {
  counter: number;
};

const defaultValue: Counter = {
  counter: 0,
};

export const store = createStore('counter-store', defaultValue, {
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
  const path = await store.$tauri.getPath();
  await open(path);
}
