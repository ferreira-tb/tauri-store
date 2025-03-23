import { create } from 'zustand';
import { open } from '@tauri-apps/plugin-shell';
import { type Option, tauri } from '@tauri-store/zustand/src/index.js';

type State = {
  counter: number;
  counter2: Option<number>;
  counter3: Option<number>;
  nested: { foo: { bar: { baz: number } } };
  nested2: Option<{ foo: { bar: Option<{ baz: number }> } }>;
  nested3: Option<{ foo: { bar: { baz: Option<number> } } }>;
};

type Action = {
  increment: () => void;
  incrementNested: () => void;
};

export const useCounterStore = create<Action & State>((set) => ({
  counter: 0,
  counter2: 0,
  counter3: 0,
  nested: { foo: { bar: { baz: 0 } } },
  nested2: { foo: { bar: { baz: 0 } } },
  nested3: { foo: { bar: { baz: 0 } } },
  increment: () => set((state) => ({ counter: state.counter + 1 })),
  incrementNested: () => {
    set((state) => {
      const baz = state.nested.foo.bar.baz;
      return {
        nested: { foo: { bar: { baz: baz + 1 } } },
      };
    });
  },
}));

export const tauriHandler = tauri('counter-store', useCounterStore, {
  filterKeys: ['counter', 'nested'],
  filterKeysStrategy: 'pick',
  saveOnExit: true,
  saveOnChange: true,
  saveStrategy: 'debounce',
  saveInterval: 1000,

  hooks: {
    beforeBackendSync: (state) => {
      state.counter2 &&= null;
      console.log(state);
      return state;
    },
    beforeFrontendSync: (state) => {
      state.nested3 = { foo: { bar: { baz: Date.now() } } };
      return state;
    },
  },
});

export async function openStore() {
  const path = await tauriHandler.getPath();
  await open(path);
}
