import { open } from '@tauri-apps/plugin-shell';
import { Store } from 'tauri-store/src/index.js';

const counter = {
  counter: 0,
  counter2: 0,
  counter3: 0,
  nested: { foo: { bar: { baz: 0 } } },
  nested2: { foo: { bar: { baz: 0 } } },
  nested3: { foo: { bar: { baz: 0 } } },
};

export const store = new Store('counter-store', counter, {
  filterKeys: ['counter', 'nested', 'nested2', 'nested3'],
  filterKeysStrategy: 'pick',
  saveOnExit: true,
  saveOnChange: true,
  saveStrategy: 'debounce',
  saveInterval: 1000,

  hooks: {
    beforeBackendSync: (state) => {
      console.log(state);
      return state;
    },
  },
});

if (!Object.hasOwn(window, '$store')) {
  Object.defineProperty(window, '$store', {
    value: store,
    writable: false,
    configurable: false,
    enumerable: true,
  });
}

export async function openStore() {
  const path = await store.getPath();
  await open(path);
}

export function increment() {
  store.update('counter', (it) => ++it);
}

export function incrementNested() {
  store.update('nested', (it) => {
    ++it.foo.bar.baz;
    return it;
  });
}
