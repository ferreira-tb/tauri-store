import { ref } from 'vue';
import { defineStore } from 'pinia';
import { open } from '@tauri-apps/plugin-shell';

type Counter = {
  counter: number;
};

function store() {
  const counter = ref<Counter['counter']>(0);

  function increment() {
    counter.value++;
  }

  return {
    counter,
    increment,
  };
}

export const useStore = defineStore('counter-store', store, {
  tauri: {
    saveStrategy: 'debounce',
    saveInterval: 1000,
    hooks: {
      beforeBackendSync: (state: Counter) => {
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
