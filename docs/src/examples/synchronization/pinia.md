```ts{10,11}
import { defineStore } from 'pinia';

function store() {
  const counter = ref(0);
  return { counter };
}

export const useStore = defineStore('store', store, {
  tauri: {
    syncStrategy: 'debounce',
    syncInterval: 1000,
  },
});
```
