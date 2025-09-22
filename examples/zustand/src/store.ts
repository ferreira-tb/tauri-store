import { create } from 'zustand';
import { open } from '@tauri-apps/plugin-shell';
import { tauri } from '@tauri-store/zustand/src/index.js';

type State = {
  counter: number;
};

type Action = {
  increment: () => void;
};

export const useCounterStore = create<Action & State>((set) => ({
  counter: 0,
  increment: () => set((state) => ({ counter: state.counter + 1 })),
}));

export const tauriHandler = tauri('counter-store', useCounterStore, {
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
  const path = await tauriHandler.getPath();
  await open(path);
}
