import { ref } from 'vue';
import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-shell';

function store() {
  const counter = ref(0);
  const counter2 = ref(0);
  const counter3 = ref(0);

  const nested = ref({ foo: { bar: { baz: 0 } } });

  function increment() {
    counter.value++;
  }

  return {
    counter,
    counter2,
    counter3,
    nested,
    increment,
  };
}

export const useStore = defineStore('counter-store', store, {
  tauri: {
    filterKeys: ['counter', 'counter2', 'nested'],
    filterKeysStrategy: 'pick',
    saveOnChange: true,
    saveStrategy: 'debounce',
    saveInterval: 1000,
  },
});

export async function openStore() {
  const path = await useStore().$tauri.getPath();
  await open(path);
}
