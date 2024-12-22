```ts{11}
import { ref } from 'vue';
import { defineStore } from 'pinia';

function store() {
  const counter = ref(0);
  return { counter };
}

export const useStore = defineStore('store', store, {
  tauri: {
    saveOnChange: true,

    // You can also debounce or throttle when saving.
    // This is optional. The default behavior is to save immediately.
    saveStrategy: 'debounce',
    saveInterval: 1000,
  },
});
```
