import { ref } from 'vue';
import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-shell';
import { counterOptions, counter as value } from 'example-shared-js/src/index.js';

function store() {
  const counter = ref(value.counter);
  const counter2 = ref(value.counter2);
  const counter3 = ref(value.counter3);
  const nested = ref(value.nested);
  const nested2 = ref(value.nested2);
  const nested3 = ref(value.nested3);

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
  tauri: counterOptions,
});

export async function openStore() {
  const path = await useStore().$tauri.getPath();
  await open(path);
}
