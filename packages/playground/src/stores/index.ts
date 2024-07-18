import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useStore = defineStore('store', () => {
  const counter = ref(0);

  function increment() {
    counter.value++;
  }

  function decrement() {
    counter.value--;
  }

  return {
    counter,
    increment,
    decrement
  };
});
