```ts{5}
import { Store } from 'tauri-plugin-svelte';

const value = { counter: 0 };
const store = new Store('store', value, {
  saveOnChange: true,

  // You can also debounce or throttle when saving.
  // This is optional. The default behavior is to save immediately.
  saveStrategy: 'debounce',
  saveInterval: 1000,
});
```
