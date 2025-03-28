import { ref } from 'vue';
import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-shell';
import type { Option } from '@tauri-store/pinia/src/index.js';

type Counter = {
  counter: number;
  counter2: Option<number>;
  nested: { foo: { bar: { baz: number } } };
};

function store() {
  const counter = ref<Counter['counter']>(0);
  const counter2 = ref<Counter['counter2']>(0);
  const nested = ref<Counter['nested']>({ foo: { bar: { baz: 0 } } });

  function increment() {
    counter.value++;
  }

  function incrementNested() {
    nested.value.foo.bar.baz++;
  }

  return {
    counter,
    counter2,
    nested,
    increment,
    incrementNested,
  };
}

export const useStore = defineStore('counter-store', store, {
  tauri: {
    saveStrategy: 'debounce',
    saveInterval: 1000,
    hooks: {
      beforeBackendSync: (state: Counter) => {
        state.counter2 = 42;
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
