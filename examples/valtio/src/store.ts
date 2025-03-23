import { deepClone } from 'valtio/utils';
import { open } from '@tauri-apps/plugin-shell';
import { type Option, store as valtio } from '@tauri-store/valtio/src/index.js';

type Counter = {
  counter: number;
  counter2: Option<number>;
  counter3: Option<number>;
  nested: { foo: { bar: { baz: number } } };
  nested2: Option<{ foo: { bar: Option<{ baz: number }> } }>;
  nested3: Option<{ foo: { bar: { baz: Option<number> } } }>;
};

const counter: Counter = {
  counter: 0,
  counter2: 0,
  counter3: 0,
  nested: { foo: { bar: { baz: 0 } } },
  nested2: { foo: { bar: { baz: 0 } } },
  nested3: { foo: { bar: { baz: 0 } } },
};

export const store = valtio('counter-store', counter, {
  filterKeys: ['counter', 'nested', 'nested2', 'nested3'],
  filterKeysStrategy: 'pick',
  saveOnExit: true,
  saveOnChange: true,
  saveStrategy: 'debounce',
  saveInterval: 1000,

  hooks: {
    beforeBackendSync: (state) => {
      state = deepClone(state);
      state.counter2 &&= null;
      if (state.nested2) {
        state.nested2.foo.bar &&= null;
      }

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
  const path = await store.getPath();
  await open(path);
}

export const increment = () => {
  store.state.counter += 1;
};

export const incrementNested = () => {
  store.state.nested.foo.bar.baz += 1;
};
