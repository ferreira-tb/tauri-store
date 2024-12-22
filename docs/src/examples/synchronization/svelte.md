```ts{5,6}
import { Store } from 'tauri-plugin-svelte';

const value = { counter: 0 };
const store = new Store('store', value, {
  syncStrategy: 'debounce',
  syncInterval: 1000,
});
```
