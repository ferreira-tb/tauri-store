import { ref } from 'vue';
import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-shell';
import type { Option } from 'tauri-plugin-pinia/src/index.js';

type Counter = {
  counter: number;
  counter2: Option<number>;
  counter3: Option<number>;
  nested: { foo: { bar: { baz: number } } };
  nested2: Option<{ foo: { bar: Option<{ baz: number }> } }>;
  nested3: Option<{ foo: { bar: { baz: Option<number> } } }>;
};

function store() {
  const counter = ref<Counter['counter']>(0);
  const counter2 = ref<Counter['counter2']>(0);
  const counter3 = ref<Counter['counter3']>(0);
  const nested = ref<Counter['nested']>({ foo: { bar: { baz: 0 } } });
  const nested2 = ref<Counter['nested2']>({ foo: { bar: { baz: 0 } } });
  const nested3 = ref<Counter['nested3']>({ foo: { bar: { baz: 0 } } });

  function increment() {
    counter.value++;
  }

  function incrementNested() {
    nested.value.foo.bar.baz++;
  }

  return {
    counter,
    counter2,
    counter3,
    nested,
    nested2,
    nested3,
    increment,
    incrementNested,
  };
}

export const useStore = defineStore('counter-store', store, {
  tauri: {
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
  },
});

export async function openStore() {
  const path = await useStore().$tauri.getPath();
  await open(path);
}
