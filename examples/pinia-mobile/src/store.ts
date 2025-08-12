import { ref } from 'vue';
import { defineStore } from 'pinia';

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

export const useStore = defineStore('counter-store', store);
