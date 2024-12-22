3. Create a store:

::: code-group

```ts{3} [src/lib/stores/counter.ts]
import { Store } from 'tauri-plugin-svelte';

const counterStore = new Store('counter', { counter: 0 });
```

:::

4. Start it:

```ts{3}
import { counterStore } from '$lib/stores/counter';

await counterStore.start();
```

::: tip
The stores won't be saved nor synchronized until you call the [`start`](https://tb.dev.br/tauri-store/reference/tauri-plugin-svelte/classes/Store.html#start) method, but they can still be used as regular Svelte stores.
:::

5. Use the store in your Svelte components:

```svelte
<script>
  import { counterStore } from '$lib/stores/counter';
</script>

<div>
  <p>Counter: {$counterStore.counter}</p>
  <button type="button" onclick={() => $counterStore.counter++}>
    <span>Increment</span>
  </button>
</div>
```
