import { ref } from 'vue';
import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-shell';
import { counterOptions, counter as value } from 'example-shared-js/src/index.js';

function store() {
  const counter = ref(value.counter);
  const counter2 = ref(value.counter2);
  const counter3 = ref(value.counter3);
  const nested = ref(value.nested);

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
  tauri: counterOptions,
});

export async function openStore() {
  const path = await useStore().$tauri.getPath();
  await open(path);
}
